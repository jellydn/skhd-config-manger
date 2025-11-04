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
   * Generates the `open` command for launching an application
   * Uses app_path instead of display_name to avoid issues with special characters
   */
  generateLaunchCommand(app: Application): string {
    // Use app_path for robust launching - handles special characters in names
    // The path is already properly formatted from the backend
    return `open "${app.app_path}"`;
  },
};
