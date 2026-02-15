import { useState, useEffect } from "react";
import { ArticleForm } from "./components/ArticleForm";
import { MarkdownEditor } from "./components/MarkdownEditor";
import { PreviewPane } from "./components/PreviewPane";
import { TemplateSelector } from "./components/TemplateSelector";
import { SettingsModal } from "./components/SettingsModal";
import { JiraTicketPicker } from "./components/JiraTicketPicker";
import { ConnectionStatus } from "./components/ConnectionStatus";
import { DraftingSpinner } from "./components/DraftingSpinner";
import { PublishDialog } from "./components/PublishDialog";
import { DraftsList } from "./components/DraftsList";
import { useDraftArticle } from "./hooks/useDraftArticle";
import { useSettingsStore } from "./stores/settingsStore";
import { useAuthStore } from "./stores/authStore";
import { parseMarkdownToArticle } from "./lib/markdownParser";
import { invoke } from "./lib/tauri";
import type { Template } from "./bindings/Template";
import type { JiraTicket } from "./bindings/JiraTicket";
import type { PublishResult } from "./bindings/PublishResult";
import type { Article } from "./bindings/Article";
import "./App.css";

function App() {
  const [markdown, setMarkdown] = useState("");
  const [selectedTemplate, setSelectedTemplate] = useState<Template | null>(null);
  const [showPreview, setShowPreview] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [showJiraPicker, setShowJiraPicker] = useState(false);
  const [loadedTicket, setLoadedTicket] = useState<JiraTicket | null>(null);
  const [showPublishDialog, setShowPublishDialog] = useState(false);
  const [articleTitle, setArticleTitle] = useState("");
  const [publishSuccess, setPublishSuccess] = useState<PublishResult | null>(null);
  const [currentArticleId, setCurrentArticleId] = useState<bigint | null>(null);
  const [showDraftsList, setShowDraftsList] = useState(false);

  const { ollamaUrl, selectedModel, confluenceUrl } = useSettingsStore();
  const { ollamaConnected, confluenceConnected } = useAuthStore();
  const draftMutation = useDraftArticle();

  const handleTemplateSelect = (template: Template) => {
    setSelectedTemplate(template);
    // Populate editor with template structure
    setMarkdown(template.output_structure);
  };

  const saveDraft = async (silent = false): Promise<Article | null> => {
    setIsSaving(true);
    try {
      const parsed = parseMarkdownToArticle(markdown);

      // Validate required fields
      if (!parsed.problem || parsed.problem.trim() === '') {
        alert('Problem description is required. Please add a "## Problem" section to your article.');
        return null;
      }

      if (!parsed.solution || parsed.solution.trim() === '') {
        alert('Solution is required. Please add a "## Solution" section to your article.');
        return null;
      }

      const newArticle = {
        ticketKey: loadedTicket?.key ?? null,
        title: parsed.title,
        problem: parsed.problem,
        solution: parsed.solution,
        expectedResult: parsed.expectedResult ?? null,
        prerequisites: parsed.prerequisites ?? null,
        additionalNotes: parsed.additionalNotes ?? null,
        tags: parsed.tags,
        contentMarkdown: markdown,
        templateId: selectedTemplate?.id ?? null,
      };

      const numericArticleId = currentArticleId !== null ? Number(currentArticleId) : null;
      const savedArticle = await invoke<Article>('save_draft', {
        article: newArticle,
        articleId: numericArticleId,
      });
      setCurrentArticleId(savedArticle.id);
      if (!silent) {
        alert(`Draft saved successfully! ID: ${savedArticle.id}`);
      }
      return savedArticle;
    } catch (error: any) {
      console.error('Failed to save:', error);
      alert(`Failed to save draft: ${error.message || error}`);
      return null;
    } finally {
      setIsSaving(false);
    }
  };

  const handleSave = async (): Promise<void> => {
    await saveDraft(false);
  };

  const handlePublish = async () => {
    if (!confluenceConnected) {
      alert('Please connect to Confluence in Settings first');
      return;
    }

    const savedArticle = await saveDraft(true);
    if (!savedArticle) {
      return;
    }

    // Extract title from markdown (first heading or first line)
    const firstLine = savedArticle.content_markdown.split('\n')[0] ?? '';
    const extractedTitle = firstLine.startsWith('#')
      ? firstLine.replace(/^#+\s*/, '').trim()
      : firstLine.trim();
    const title = extractedTitle || savedArticle.title || 'Untitled Article';

    setArticleTitle(title);
    setShowPublishDialog(true);
  };

  const handlePublishSuccess = (result: PublishResult) => {
    setPublishSuccess(result);
    setShowPublishDialog(false);
  };

  const handleJiraTicketSelect = (ticket: JiraTicket) => {
    setLoadedTicket(ticket);
    // Populate the markdown editor with ticket data
    // Title from summary, problem from description, solution from last comment
    const lastComment = ticket.comments.length > 0
      ? ticket.comments[ticket.comments.length - 1].body
      : '';

    const ticketMarkdown = `# ${ticket.summary}

## Problem
${ticket.description ?? '[No description provided in ticket]'}

## Solution
${lastComment ?? '[No resolution note found in ticket comments]'}

## Tags
${ticket.labels.join(', ')}`;

    setMarkdown(ticketMarkdown);
  };

  const handleDraftWithAI = async () => {
    if (!loadedTicket) {
      alert('Please load a Jira ticket first');
      return;
    }

    if (!selectedTemplate) {
      alert('Please select a template first');
      return;
    }

    if (!ollamaConnected) {
      alert('Ollama is not connected. Please start Ollama and refresh the connection status.');
      return;
    }

    try {
      const generatedMarkdown = await draftMutation.mutateAsync({
        ticket: loadedTicket,
        templateId: selectedTemplate.id,
        ollamaUrl,
        model: selectedModel,
      });

      setMarkdown(generatedMarkdown);
    } catch (error: any) {
      console.error('Failed to draft article:', error);
      alert(`Failed to generate article: ${error.message || error}`);
    }
  };

  const handleCancelDrafting = () => {
    // TanStack Query doesn't have built-in abort, but we can just ignore the result
    // The mutation will complete but we won't use it
    alert('Cancellation is not yet implemented. Please wait for generation to complete.');
  };

  const handleLoadDraft = (article: Article) => {
    setMarkdown(article.content_markdown);
    setCurrentArticleId(article.id);
    if (article.ticket_key) {
      // Could optionally reload the full ticket here
    }
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Cmd+S / Ctrl+S - Save draft
      if ((e.metaKey || e.ctrlKey) && e.key === 's') {
        e.preventDefault();
        handleSave();
      }
      // Cmd+Shift+P / Ctrl+Shift+P - Toggle preview
      else if ((e.metaKey || e.ctrlKey) && e.shiftKey && e.key === 'p') {
        e.preventDefault();
        setShowPreview(!showPreview);
      }
      // Cmd+D / Ctrl+D - Open drafts list
      else if ((e.metaKey || e.ctrlKey) && e.key === 'd') {
        e.preventDefault();
        setShowDraftsList(true);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [showPreview, handleSave]); // Include all dependencies used in handlers

  return (
    <div className="h-screen flex flex-col">
      <header className="bg-gray-800 text-white p-4">
        <div className="flex justify-between items-center">
          <div className="flex items-center gap-4">
            <h1 className="text-2xl font-bold">KB Article Drafter</h1>
            <ConnectionStatus />
          </div>
          <div className="flex gap-2">
            <button
              onClick={() => setShowDraftsList(true)}
              className="px-3 py-1 bg-gray-700 rounded hover:bg-gray-600"
            >
              📄 My Drafts
            </button>
            <button
              onClick={() => setShowJiraPicker(true)}
              className="px-3 py-1 bg-purple-600 rounded hover:bg-purple-500"
            >
              Load from Jira
            </button>
            <button
              onClick={handleDraftWithAI}
              disabled={!loadedTicket || !selectedTemplate || !ollamaConnected || draftMutation.isPending}
              className="px-3 py-1 bg-indigo-600 rounded hover:bg-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
              title={
                !loadedTicket
                  ? 'Load a Jira ticket first'
                  : !selectedTemplate
                  ? 'Select a template first'
                  : !ollamaConnected
                  ? 'Ollama is not connected'
                  : 'Generate article with AI'
              }
            >
              ✨ Draft with AI
            </button>
            <button
              onClick={() => setShowPreview(!showPreview)}
              className="px-3 py-1 bg-gray-700 rounded hover:bg-gray-600"
            >
              {showPreview ? 'Hide' : 'Show'} Preview
            </button>
            <button
              onClick={handleSave}
              disabled={isSaving}
              className="px-4 py-1 bg-blue-600 rounded hover:bg-blue-500 disabled:opacity-50"
            >
              {isSaving ? 'Saving...' : 'Save Draft'}
            </button>
            <button
              onClick={handlePublish}
              className="px-4 py-1 bg-green-600 rounded hover:bg-green-500"
            >
              Publish
            </button>
            <button
              onClick={() => setShowSettings(true)}
              className="px-3 py-1 bg-gray-700 rounded hover:bg-gray-600"
            >
              ⚙️ Settings
            </button>
          </div>
        </div>
      </header>

      <div className="flex-1 flex overflow-hidden">
        {/* Left sidebar - Form */}
        <div className="w-96 border-r overflow-auto bg-gray-50">
          <div className="p-4">
            <TemplateSelector
              onSelect={handleTemplateSelect}
              selectedTemplateId={selectedTemplate?.id || null}
            />
            {loadedTicket && (
              <div className="mt-2 p-2 bg-blue-50 border border-blue-200 rounded text-sm">
                <span className="font-medium">Loaded from:</span> {loadedTicket.key}
              </div>
            )}
          </div>
          <ArticleForm onContentChange={setMarkdown} />
        </div>

        {/* Middle - Editor */}
        <div className="flex-1 flex overflow-hidden">
          <div className={showPreview ? 'w-1/2 border-r' : 'w-full'}>
            <div className="h-full overflow-auto">
              <MarkdownEditor
                value={markdown}
                onChange={setMarkdown}
                theme="light"
                className="h-full"
              />
            </div>
          </div>

          {/* Right - Preview */}
          {showPreview && (
            <div className="w-1/2 overflow-auto p-6 bg-white">
              <PreviewPane markdown={markdown} />
            </div>
          )}
        </div>
      </div>

      <SettingsModal
        isOpen={showSettings}
        onClose={() => setShowSettings(false)}
      />

      <JiraTicketPicker
        isOpen={showJiraPicker}
        onClose={() => setShowJiraPicker(false)}
        onSelectTicket={handleJiraTicketSelect}
      />

      {draftMutation.isPending && (
        <DraftingSpinner onCancel={handleCancelDrafting} />
      )}

      <DraftsList
        isOpen={showDraftsList}
        onClose={() => setShowDraftsList(false)}
        onLoadDraft={handleLoadDraft}
      />

      <PublishDialog
        isOpen={showPublishDialog}
        onClose={() => setShowPublishDialog(false)}
        articleId={currentArticleId}
        markdown={markdown}
        articleTitle={articleTitle}
        confluenceUrl={confluenceUrl}
        onPublishSuccess={handlePublishSuccess}
      />

      {publishSuccess && (
        <div className="fixed bottom-4 right-4 bg-green-600 text-white p-4 rounded-lg shadow-lg max-w-md">
          <div className="flex justify-between items-start">
            <div>
              <p className="font-semibold mb-1">✓ Published Successfully!</p>
              <a
                href={publishSuccess.url}
                target="_blank"
                rel="noopener noreferrer"
                className="text-sm underline hover:text-green-100"
              >
                View in Confluence →
              </a>
            </div>
            <button
              onClick={() => setPublishSuccess(null)}
              className="text-white hover:text-green-100 text-xl"
            >
              ×
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
