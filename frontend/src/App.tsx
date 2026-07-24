import { useState } from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import { Leaf, Wallet } from 'lucide-react';
import { requestAccess } from '@stellar/freighter-api';
import Marketplace from './pages/Marketplace';
import Portfolio from './pages/Portfolio';
import AdminDashboard from './pages/AdminDashboard';
import './index.css';

function App() {
  const [address, setAddress] = useState<string | null>(null);

  const connectWallet = async () => {
    try {
      let publicKey = await requestAccess();
      // @ts-ignore
      if (publicKey.error) {
         // @ts-ignore
         console.error(publicKey.error);
         return;
      }
      // @ts-ignore
      setAddress(typeof publicKey === 'string' ? publicKey : publicKey.address);
    } catch (e) {
      console.error("Wallet connection failed:", e);
    }
  };

  return (
    <Router>
      <div className="app-container">
        <nav className="navbar">
          <div className="nav-brand">
            <Leaf size={28} color="#10B981" />
            <Link to="/" style={{ color: 'inherit', textDecoration: 'none' }}>GreenChain</Link>
          </div>
          
          <div className="nav-links">
            <Link to="/" className="nav-link">Marketplace</Link>
            <Link to="/portfolio" className="nav-link">My Portfolio</Link>
            <Link to="/admin" className="nav-link">Verifier Hub</Link>
          </div>

          <button className="connect-btn" onClick={connectWallet}>
            {address ? (
              <span style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <Wallet size={18} /> {address.slice(0, 4)}...{address.slice(-4)}
              </span>
            ) : (
              'Connect Freighter'
            )}
          </button>
        </nav>

        <main className="main-content">
          <Routes>
            <Route path="/" element={<Marketplace address={address} />} />
            <Route path="/portfolio" element={<Portfolio address={address} />} />
            <Route path="/admin" element={<AdminDashboard address={address} />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}

export default App;
