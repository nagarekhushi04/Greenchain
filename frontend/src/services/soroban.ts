import { rpc, Contract, TransactionBuilder, Networks, scValToNative, nativeToScVal } from '@stellar/stellar-sdk';
import { signTransaction } from '@stellar/freighter-api';

export const RPC_URL = import.meta.env.VITE_RPC_URL || 'https://soroban-testnet.stellar.org:443';
export const NETWORK_PASSPHRASE = import.meta.env.VITE_NETWORK_PASSPHRASE || Networks.TESTNET;

export const REGISTRY_CONTRACT_ID = import.meta.env.VITE_REGISTRY_CONTRACT_ID || 'CAAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQC526';
export const TOKEN_CONTRACT_ID = import.meta.env.VITE_TOKEN_CONTRACT_ID || 'CABAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAFNSZ';
export const MARKETPLACE_CONTRACT_ID = import.meta.env.VITE_MARKETPLACE_CONTRACT_ID || 'CABQGAYDAMBQGAYDAMBQGAYDAMBQGAYDAMBQGAYDAMBQGAYDAMBQGCK3';

export const server = new rpc.Server(RPC_URL);

/**
 * Ensure a Stellar Testnet account exists and is funded via Friendbot.
 */
export async function ensureAccountFunded(address: string) {
  try {
    return await server.getAccount(address);
  } catch (e: any) {
    console.log(`Account ${address} not found on Testnet. Auto-funding via Stellar Friendbot...`);
    try {
      const res = await fetch(`https://friendbot.stellar.org/?addr=${encodeURIComponent(address)}`);
      if (res.ok) {
        console.log(`Account ${address} successfully funded via Friendbot.`);
        // Brief pause for ledger indexing
        await new Promise(resolve => setTimeout(resolve, 2000));
        return await server.getAccount(address);
      }
    } catch (friendbotErr) {
      console.warn("Friendbot auto-funding attempt failed:", friendbotErr);
    }
    throw new Error(`Account ${address.slice(0, 4)}...${address.slice(-4)} is not funded on Stellar Testnet. Please fund it using Stellar Friendbot or the Freighter testnet faucet.`);
  }
}

/**
 * Invoke a Soroban smart contract method.
 */
export async function invokeContract({
  contractId,
  method,
  args = [],
  userAddress,
}: {
  contractId: string;
  method: string;
  args?: any[];
  userAddress: string;
}) {
  try {
    let contract: Contract;
    try {
      contract = new Contract(contractId);
    } catch (e: any) {
      throw new Error(`Invalid Soroban Contract ID "${contractId}". Please check your deployed contract configuration.`);
    }

    const account = await ensureAccountFunded(userAddress);
    const scArgs = args.map(arg => nativeToScVal(arg));

    const tx = new TransactionBuilder(account, {
      fee: '10000',
      networkPassphrase: NETWORK_PASSPHRASE,
    })
      .addOperation(contract.call(method, ...scArgs))
      .setTimeout(30)
      .build();

    let preparedTx;
    try {
      preparedTx = await server.prepareTransaction(tx);
    } catch (prepErr: any) {
      console.warn("Prepare transaction error (contract may not be initialized on testnet):", prepErr);
      throw new Error(`Contract interaction failed on Testnet: ${prepErr.message || prepErr}`);
    }

    const signedResult: any = await signTransaction(preparedTx.toXDR(), {
      networkPassphrase: NETWORK_PASSPHRASE,
    });

    if (!signedResult || signedResult.error) {
      throw new Error(signedResult?.error || 'User declined to sign transaction.');
    }

    const signedXdr = typeof signedResult === 'string' ? signedResult : (signedResult.signedTxXdr || signedResult);

    const txToSubmit = TransactionBuilder.fromXDR(signedXdr, NETWORK_PASSPHRASE);
    const sendResponse = await server.sendTransaction(txToSubmit);

    if (sendResponse.status === 'PENDING') {
      let statusResponse = await server.getTransaction(sendResponse.hash);
      while (statusResponse.status === 'NOT_FOUND') {
        await new Promise(resolve => setTimeout(resolve, 1000));
        statusResponse = await server.getTransaction(sendResponse.hash);
      }
      return statusResponse;
    }

    return sendResponse;
  } catch (error: any) {
    console.error(`Error invoking ${method} on ${contractId}:`, error);
    throw error;
  }
}

/**
 * Read-only simulation of a contract call (e.g. balance checks).
 */
export async function readContractState({
  contractId,
  method,
  args = [],
  userAddress,
}: {
  contractId: string;
  method: string;
  args?: any[];
  userAddress: string;
}) {
  try {
    let contract: Contract;
    try {
      contract = new Contract(contractId);
    } catch (e: any) {
      console.warn(`Invalid contract ID ${contractId} for ${method}`);
      return null;
    }

    const account = await ensureAccountFunded(userAddress);
    const scArgs = args.map(arg => nativeToScVal(arg));

    const tx = new TransactionBuilder(account, {
      fee: '10000',
      networkPassphrase: NETWORK_PASSPHRASE,
    })
      .addOperation(contract.call(method, ...scArgs))
      .setTimeout(30)
      .build();

    const simRes = await server.simulateTransaction(tx);
    if (rpc.Api.isSimulationSuccess(simRes) && simRes.result) {
      return scValToNative(simRes.result.retval);
    }
    return null;
  } catch (error) {
    console.warn(`Error reading contract ${method}:`, error);
    return null;
  }
}


