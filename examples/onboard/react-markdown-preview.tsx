import { DxMarkdown } from "@/components/content/markdown";

const launchNotesMarkdown = [
  "## Launch notes",
  "",
  "- Keep raw HTML disabled unless a reviewed content policy allows it.",
  "- Treat external links and plugin changes as production review points.",
  "- Keep release copy in source control until a CMS owner is assigned.",
].join("\n");

export function LaunchMarkdownPreview() {
  return (
    <section className="grid gap-3 rounded-md border p-4">
      <DxMarkdown skipHtml>{launchNotesMarkdown}</DxMarkdown>
    </section>
  );
}
