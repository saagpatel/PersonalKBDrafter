import { useState, useEffect } from 'react';
import { invoke } from '../lib/tauri';
import type { ConfluenceSpace } from '../bindings/ConfluenceSpace';
import type { PublishResult } from '../bindings/PublishResult';
import type { FlaggedSection } from '../bindings/FlaggedSection';

interface Props {
  isOpen: boolean;
  onClose: () => void;
  articleId: bigint | null;
  markdown: string;
  articleTitle: string;
  confluenceUrl: string;
  onPublishSuccess: (result: PublishResult) => void;
}

export function PublishDialog({
  isOpen,
  onClose,
  articleId,
  markdown,
  articleTitle,
  confluenceUrl,
  onPublishSuccess,
}: Props) {
  const [spaces, setSpaces] = useState<ConfluenceSpace[]>([]);
  const [selectedSpace, setSelectedSpace] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const [publishing, setPublishing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [sensitiveFlags, setSensitiveFlags] = useState<FlaggedSection[]>([]);
  const [checkingSensitive, setCheckingSensitive] = useState(false);

  useEffect(() => {
    if (isOpen) {
      loadSpaces();
      checkSensitiveData();
    }
  }, [isOpen]);

  const loadSpaces = async () => {
    setLoading(true);
    setError(null);
    try {
      const spaceList = await invoke<ConfluenceSpace[]>('list_confluence_spaces', {
        confluenceUrl,
      });
      setSpaces(spaceList);
    } catch (err: any) {
      setError(`Failed to load spaces: ${err.message || err}`);
    } finally {
      setLoading(false);
    }
  };

  const checkSensitiveData = async () => {
    setCheckingSensitive(true);
    try {
      const flags = await invoke<FlaggedSection[]>('scan_sensitive_data', {
        content: markdown,
      });
      setSensitiveFlags(flags);
    } catch (err) {
      console.error('Failed to scan sensitive data:', err);
    } finally {
      setCheckingSensitive(false);
    }
  };

  const handlePublish = async () => {
    if (!selectedSpace) {
      alert('Please select a Confluence space');
      return;
    }

    if (articleId === null) {
      setError('Please save your draft before publishing.');
      return;
    }

    if (sensitiveFlags.length > 0) {
      const confirmed = window.confirm(
        `Found ${sensitiveFlags.length} potential sensitive data issue(s). Do you want to proceed anyway? Review the warnings below before continuing.`
      );
      if (!confirmed) return;
    }

    setPublishing(true);
    setError(null);

    try {
      const numericArticleId = Number(articleId);
      if (!Number.isSafeInteger(numericArticleId)) {
        throw new Error('Article ID is too large to publish safely.');
      }

      const result = await invoke<PublishResult>('publish_article', {
        articleId: numericArticleId,
        spaceKey: selectedSpace,
        confluenceUrl,
      });
      onPublishSuccess(result);
    } catch (err: any) {
      setError(`Failed to publish: ${err.message || err}`);
    } finally {
      setPublishing(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 max-w-2xl w-full max-h-[80vh] overflow-auto">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-2xl font-bold">Publish to Confluence</h2>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-gray-700 text-2xl"
          >
            ×
          </button>
        </div>

        {/* Article Info */}
        <div className="mb-4 p-3 bg-gray-50 rounded">
          <p className="text-sm font-medium">Article: {articleTitle || '(Untitled)'}</p>
        </div>

        {/* Sensitive Data Warnings */}
        {checkingSensitive ? (
          <div className="mb-4 p-3 bg-blue-50 rounded text-sm">
            Scanning for sensitive data...
          </div>
        ) : sensitiveFlags.length > 0 ? (
          <div className="mb-4 p-3 bg-yellow-50 border border-yellow-200 rounded">
            <p className="text-sm font-semibold text-yellow-800 mb-2">
              ⚠️ Warning: {sensitiveFlags.length} potential sensitive data issue(s) detected
            </p>
            <div className="space-y-1 max-h-32 overflow-auto">
              {sensitiveFlags.slice(0, 5).map((flag, idx) => (
                <div key={idx} className="text-xs text-yellow-700">
                  Line {flag.line_number}: {flag.pattern_type} - "{flag.matched_text}"
                </div>
              ))}
              {sensitiveFlags.length > 5 && (
                <p className="text-xs text-yellow-700">
                  ...and {sensitiveFlags.length - 5} more
                </p>
              )}
            </div>
          </div>
        ) : (
          <div className="mb-4 p-3 bg-green-50 border border-green-200 rounded text-sm text-green-700">
            ✓ No sensitive data detected
          </div>
        )}

        {/* Space Selection */}
        <div className="mb-4">
          <label htmlFor="space" className="block text-sm font-medium mb-2">
            Select Confluence Space
          </label>
          {loading ? (
            <p className="text-sm text-gray-500">Loading spaces...</p>
          ) : error ? (
            <div className="text-sm text-red-600 mb-2">{error}</div>
          ) : (
            <select
              id="space"
              value={selectedSpace}
              onChange={(e) => setSelectedSpace(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md"
            >
              <option value="">-- Select a space --</option>
              {spaces.map((space) => (
                <option key={space.key} value={space.key}>
                  {space.name} ({space.key})
                </option>
              ))}
            </select>
          )}
        </div>
        {/* Actions */}
        <div className="flex justify-end gap-2">
          <button
            onClick={onClose}
            className="px-4 py-2 border border-gray-300 rounded hover:bg-gray-50"
          >
            Cancel
          </button>
          <button
            onClick={handlePublish}
            disabled={publishing || !selectedSpace || loading}
            className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50"
          >
            {publishing ? 'Publishing...' : 'Publish to Confluence'}
          </button>
        </div>
      </div>
    </div>
  );
}
