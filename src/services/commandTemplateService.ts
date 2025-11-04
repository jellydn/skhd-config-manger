import { invoke } from '@tauri-apps/api/core';
import type { CommandCategory, CommandTemplate } from '../types';

export const commandTemplateService = {
  /**
   * Get all command templates, optionally filtered by category
   */
  async listTemplates(categoryId?: string): Promise<CommandTemplate[]> {
    return invoke('get_command_templates', { categoryId });
  },

  /**
   * Get all command categories
   */
  async listCategories(): Promise<CommandCategory[]> {
    return invoke('get_command_categories');
  },

  /**
   * Generate a command from a template with parameter values
   */
  async generateCommand(
    templateId: string,
    parameterValues: Record<string, string>
  ): Promise<string> {
    return invoke('generate_command_from_template', {
      templateId,
      parameterValues,
    });
  },

  /**
   * Search templates by name or description
   */
  searchTemplates(templates: CommandTemplate[], query: string): CommandTemplate[] {
    if (!query.trim()) return templates;

    const lowerQuery = query.toLowerCase();
    return templates.filter(
      (template) =>
        template.name.toLowerCase().includes(lowerQuery) ||
        template.description.toLowerCase().includes(lowerQuery)
    );
  },

  /**
   * Group templates by category
   */
  groupByCategory(
    templates: CommandTemplate[],
    categories: CommandCategory[]
  ): Map<CommandCategory, CommandTemplate[]> {
    const grouped = new Map<CommandCategory, CommandTemplate[]>();

    categories.forEach((category) => {
      const categoryTemplates = templates.filter((t) => t.category_id === category.id);
      if (categoryTemplates.length > 0) {
        grouped.set(category, categoryTemplates);
      }
    });

    return grouped;
  },
};
