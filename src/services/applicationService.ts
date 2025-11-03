import { invoke } from '@tauri-apps/api/core';
import type { Application } from '../types';

export const applicationService = {
  /**
   * Fetches all installed macOS applications
   */
  async listApplications(): Promise<Application[]> {
    return invoke('get_installed_applications');
  },

  /**
   * Filters applications by search query (display name)
   */
  searchApplications(apps: Application[], query: string): Application[] {
    if (!query.trim()) {
      return apps;
    }
    const lowerQuery = query.toLowerCase();
    return apps.filter((app) =>
      app.display_name.toLowerCase().includes(lowerQuery)
    );
  },

  /**
   * Generates the `open -a` command for launching an application
   */
  generateLaunchCommand(app: Application): string {
    return `open -a "${app.display_name}"`;
  },
};
