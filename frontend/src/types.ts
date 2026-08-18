export interface Project {
  id: number;
  project_id: number;
  name: string;
  verified: boolean;
}

export interface Listing {
  id: number;
  listing_id: number;
  seller: string;
  asset_token: string;
  amount: number;
  price_per_token: number;
}
