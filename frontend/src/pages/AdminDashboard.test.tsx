import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import AdminDashboard from './AdminDashboard';

describe('AdminDashboard Component', () => {
  it('renders Verifier Dashboard heading', () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ projects: [] }),
    });

    render(<AdminDashboard address="0x123" />);
    
    expect(screen.getByText('Verifier Dashboard')).toBeInTheDocument();
    expect(screen.getByText('Register New Project')).toBeInTheDocument();
    expect(screen.getByText('Registered Projects')).toBeInTheDocument();
  });

  it('renders no projects message when list is empty', async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ projects: [] }),
    });

    render(<AdminDashboard address="0x123" />);
    expect(await screen.findByText('No projects found.')).toBeInTheDocument();
  });
});
