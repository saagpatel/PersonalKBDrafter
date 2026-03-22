import { useState, useEffect } from "react";
import { invoke } from "../lib/tauri";
import { useAuthStore } from "../stores/authStore";
import { useSettingsStore } from "../stores/settingsStore";

interface Props {
  isOpen: boolean;
  onClose: () => void;
}

export function SettingsModal({ isOpen, onClose }: Props) {
  const [jiraUrl, setJiraUrl] = useState("");
  const [jiraPat, setJiraPat] = useState("");
  const [testingJira, setTestingJira] = useState(false);
  const [jiraTestResult, setJiraTestResult] = useState<
    "success" | "error" | null
  >(null);

  const [confluenceUrl, setConfluenceUrl] = useState("");
  const [confluencePat, setConfluencePat] = useState("");
  const [testingConfluence, setTestingConfluence] = useState(false);
  const [confluenceTestResult, setConfluenceTestResult] = useState<
    "success" | "error" | null
  >(null);

  const {
    ollamaUrl,
    setOllamaUrl,
    selectedModel,
    setSelectedModel,
    jiraUrl: storedJiraUrl,
    setJiraUrl: persistJiraUrl,
    confluenceUrl: storedConfluenceUrl,
    setConfluenceUrl: persistConfluenceUrl,
  } = useSettingsStore();
  const {
    jiraConnected,
    setJiraConnected,
    confluenceConnected,
    setConfluenceConnected,
  } = useAuthStore();

  // Initialize local state from stored settings
  useEffect(() => {
    if (storedJiraUrl) {
      setJiraUrl(storedJiraUrl);
    }
  }, [storedJiraUrl]);

  useEffect(() => {
    if (storedConfluenceUrl) {
      setConfluenceUrl(storedConfluenceUrl);
    }
  }, [storedConfluenceUrl]);

  useEffect(() => {
    if (isOpen) {
      // Reset test result when modal opens
      setJiraTestResult(null);
    }
  }, [isOpen]);

  const handleTestJira = async () => {
    if (!jiraUrl || !jiraPat) {
      alert("Please enter both Jira URL and PAT");
      return;
    }

    setTestingJira(true);
    setJiraTestResult(null);

    try {
      const result = await invoke<boolean>("test_jira_connection", {
        baseUrl: jiraUrl,
        pat: jiraPat,
      });

      if (result) {
        setJiraTestResult("success");
        // Save configuration
        await invoke("save_jira_config", {
          baseUrl: jiraUrl,
          pat: jiraPat,
        });
        persistJiraUrl(jiraUrl);
        setJiraConnected(true, jiraUrl);
        setJiraPat(""); // Clear PAT from state for security
      }
    } catch (error) {
      console.error("Jira connection test failed:", error);
      setJiraTestResult("error");
    } finally {
      setTestingJira(false);
    }
  };

  const handleDisconnectJira = async () => {
    try {
      await invoke("disconnect_jira");
      persistJiraUrl("");
      setJiraConnected(false);
      setJiraUrl("");
      setJiraPat("");
      setJiraTestResult(null);
    } catch (error) {
      console.error("Failed to disconnect Jira:", error);
    }
  };

  const handleTestConfluence = async () => {
    if (!confluenceUrl || !confluencePat) {
      alert("Please enter both Confluence URL and PAT");
      return;
    }

    setTestingConfluence(true);
    setConfluenceTestResult(null);

    try {
      const result = await invoke<boolean>("test_confluence_connection", {
        baseUrl: confluenceUrl,
        pat: confluencePat,
      });

      if (result) {
        setConfluenceTestResult("success");
        // Save configuration
        await invoke("save_confluence_config", {
          baseUrl: confluenceUrl,
          pat: confluencePat,
        });
        // Persist URL to settings store
        persistConfluenceUrl(confluenceUrl);
        setConfluenceConnected(true, confluenceUrl);
        setConfluencePat(""); // Clear PAT from state for security
      }
    } catch (error) {
      console.error("Confluence connection test failed:", error);
      setConfluenceTestResult("error");
    } finally {
      setTestingConfluence(false);
    }
  };

  const handleDisconnectConfluence = async () => {
    try {
      await invoke("disconnect_confluence");
      setConfluenceConnected(false);
      persistConfluenceUrl("");
      setConfluenceUrl("");
      setConfluencePat("");
      setConfluenceTestResult(null);
    } catch (error) {
      console.error("Failed to disconnect Confluence:", error);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 max-w-2xl w-full max-h-[80vh] overflow-auto">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-2xl font-bold">Settings</h2>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-gray-700 text-2xl"
          >
            ×
          </button>
        </div>

        <div className="space-y-6">
          {/* Jira Configuration */}
          <section>
            <h3 className="text-lg font-semibold mb-3 flex items-center gap-2">
              Jira Data Center
              {jiraConnected && (
                <span className="text-sm px-2 py-0.5 bg-green-100 text-green-800 rounded">
                  Connected
                </span>
              )}
            </h3>

            {!jiraConnected ? (
              <div className="space-y-3">
                <div>
                  <label
                    htmlFor="jiraUrl"
                    className="block text-sm font-medium mb-1"
                  >
                    Jira Base URL
                  </label>
                  <input
                    id="jiraUrl"
                    type="text"
                    value={jiraUrl}
                    onChange={(e) => setJiraUrl(e.target.value)}
                    placeholder="https://jira.yourcompany.com"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>

                <div>
                  <label
                    htmlFor="jiraPat"
                    className="block text-sm font-medium mb-1"
                  >
                    Personal Access Token
                  </label>
                  <input
                    id="jiraPat"
                    type="password"
                    value={jiraPat}
                    onChange={(e) => setJiraPat(e.target.value)}
                    placeholder="Your Jira PAT"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                  <p className="text-xs text-gray-500 mt-1">
                    Generate in Jira: Profile → Personal Access Tokens
                  </p>
                </div>

                <button
                  onClick={handleTestJira}
                  disabled={testingJira || !jiraUrl || !jiraPat}
                  className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
                >
                  {testingJira ? "Testing..." : "Test & Save Connection"}
                </button>

                {jiraTestResult === "success" && (
                  <p className="text-sm text-green-600">
                    ✓ Connection successful!
                  </p>
                )}
                {jiraTestResult === "error" && (
                  <p className="text-sm text-red-600">
                    ✗ Connection failed. Check URL and PAT.
                  </p>
                )}
              </div>
            ) : (
              <div>
                <p className="text-sm text-gray-600 mb-2">
                  Connected to Jira. Your PAT is stored securely in the system
                  keychain.
                </p>
                <button
                  onClick={handleDisconnectJira}
                  className="px-4 py-2 border border-red-600 text-red-600 rounded hover:bg-red-50"
                >
                  Disconnect
                </button>
              </div>
            )}
          </section>

          {/* Confluence Configuration */}
          <section>
            <h3 className="text-lg font-semibold mb-3 flex items-center gap-2">
              Confluence Data Center
              {confluenceConnected && (
                <span className="text-sm px-2 py-0.5 bg-green-100 text-green-800 rounded">
                  Connected
                </span>
              )}
            </h3>

            {!confluenceConnected ? (
              <div className="space-y-3">
                <div>
                  <label
                    htmlFor="confluenceUrl"
                    className="block text-sm font-medium mb-1"
                  >
                    Confluence Base URL
                  </label>
                  <input
                    id="confluenceUrl"
                    type="text"
                    value={confluenceUrl}
                    onChange={(e) => setConfluenceUrl(e.target.value)}
                    placeholder="https://confluence.yourcompany.com"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>

                <div>
                  <label
                    htmlFor="confluencePat"
                    className="block text-sm font-medium mb-1"
                  >
                    Personal Access Token
                  </label>
                  <input
                    id="confluencePat"
                    type="password"
                    value={confluencePat}
                    onChange={(e) => setConfluencePat(e.target.value)}
                    placeholder="Your Confluence PAT"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                  <p className="text-xs text-gray-500 mt-1">
                    Generate in Confluence: Profile → Personal Access Tokens
                  </p>
                </div>

                <button
                  onClick={handleTestConfluence}
                  disabled={
                    testingConfluence || !confluenceUrl || !confluencePat
                  }
                  className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
                >
                  {testingConfluence ? "Testing..." : "Test & Save Connection"}
                </button>

                {confluenceTestResult === "success" && (
                  <p className="text-sm text-green-600">
                    ✓ Connection successful!
                  </p>
                )}
                {confluenceTestResult === "error" && (
                  <p className="text-sm text-red-600">
                    ✗ Connection failed. Check URL and PAT.
                  </p>
                )}
              </div>
            ) : (
              <div>
                <p className="text-sm text-gray-600 mb-2">
                  Connected to Confluence. Your PAT is stored securely in the
                  system keychain.
                </p>
                <button
                  onClick={handleDisconnectConfluence}
                  className="px-4 py-2 border border-red-600 text-red-600 rounded hover:bg-red-50"
                >
                  Disconnect
                </button>
              </div>
            )}
          </section>

          {/* Ollama Configuration */}
          <section>
            <h3 className="text-lg font-semibold mb-3">Ollama (Local LLM)</h3>
            <div className="space-y-3">
              <div>
                <label
                  htmlFor="ollamaUrl"
                  className="block text-sm font-medium mb-1"
                >
                  Ollama URL
                </label>
                <input
                  id="ollamaUrl"
                  type="text"
                  value={ollamaUrl}
                  onChange={(e) => setOllamaUrl(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md"
                />
              </div>

              <div>
                <label
                  htmlFor="model"
                  className="block text-sm font-medium mb-1"
                >
                  Model
                </label>
                <input
                  id="model"
                  type="text"
                  value={selectedModel}
                  onChange={(e) => setSelectedModel(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md"
                  placeholder="llama3.2"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Model must be pulled in Ollama first (e.g., ollama pull
                  llama3.2)
                </p>
              </div>
            </div>
          </section>
        </div>

        <div className="mt-6 flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-gray-800 text-white rounded hover:bg-gray-700"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
