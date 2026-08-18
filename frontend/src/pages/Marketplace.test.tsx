import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import Marketplace from './Marketplace';

describe('Marketplace Component', () => {
  it('renders hero section and heading', () => {
    // Mock fetch for this test
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ listings: [] }),
    });

    render(<Marketplace address={null} />);
    
    expect(screen.getByText(/Tokenize. Trade./i)).toBeInTheDocument();
    expect(screen.getByText('Active Listings')).toBeInTheDocument();
  });

  it('displays loading state initially', () => {
    // Mock fetch to not resolve immediately
    global.fetch = vi.fn().mockReturnValue(new Promise(() => {}));

    render(<Marketplace address={null} />);
    expect(screen.getByText(/Loading live marketplace data.../i)).toBeInTheDocument();
  });
});
