import { useRef, useEffect } from 'preact/hooks';

interface ContextEditorProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
}

export function ContextEditor({ value, onChange, placeholder = 'Add notes, context, or details...' }: ContextEditorProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  // Auto-resize textarea
  useEffect(() => {
    const textarea = textareaRef.current;
    if (textarea) {
      textarea.style.height = 'auto';
      textarea.style.height = `${Math.max(100, textarea.scrollHeight)}px`;
    }
  }, [value]);

  return (
    <textarea
      ref={textareaRef}
      value={value}
      onInput={(e) => onChange((e.target as HTMLTextAreaElement).value)}
      placeholder={placeholder}
      class="font-mono text-sm"
      style={{
        flex: 1,
        resize: 'none',
        minHeight: '100px',
      }}
    />
  );
}
