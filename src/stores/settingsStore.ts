import { create } from "zustand";
import { persist } from "zustand/middleware";

interface SettingsStore {
  ollamaUrl: string;
  selectedModel: string;
  jiraUrl: string;
  confluenceUrl: string;
  theme: "light" | "dark" | "system";
  defaultSpaceKey: string | null;
  defaultTemplateId: string | null;
  setOllamaUrl: (url: string) => void;
  setSelectedModel: (model: string) => void;
  setJiraUrl: (url: string) => void;
  setConfluenceUrl: (url: string) => void;
  setTheme: (theme: "light" | "dark" | "system") => void;
  setDefaultSpaceKey: (key: string | null) => void;
  setDefaultTemplateId: (id: string | null) => void;
}

export const useSettingsStore = create<SettingsStore>()(
  persist(
    (set) => ({
      ollamaUrl: "http://localhost:11434",
      selectedModel: "llama3.2",
      jiraUrl: "",
      confluenceUrl: "",
      theme: "system",
      defaultSpaceKey: null,
      defaultTemplateId: null,

      setOllamaUrl: (url) => set({ ollamaUrl: url }),
      setSelectedModel: (model) => set({ selectedModel: model }),
      setJiraUrl: (url) => set({ jiraUrl: url }),
      setConfluenceUrl: (url) => set({ confluenceUrl: url }),
      setTheme: (theme) => set({ theme }),
      setDefaultSpaceKey: (key) => set({ defaultSpaceKey: key }),
      setDefaultTemplateId: (id) => set({ defaultTemplateId: id }),
    }),
    {
      name: "kb-drafter-settings",
    },
  ),
);
