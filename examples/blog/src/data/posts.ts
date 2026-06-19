export interface Post {
    slug: string;
    title: string;
    excerpt: string;
    content: string;
    date: string;
    author: string;
    tags?: string[];
}

export const posts: Post[] = [
    {
        slug: 'getting-started-with-dx-www',
        title: 'Getting Started with dx-www',
        excerpt: 'Learn how to build your first application with dx-www, the high-performance web framework.',
        content: `dx-www is a Rust-owned web framework with React-shaped TSX authoring, source-owned tooling, and receipts. In this post, we'll walk through setting up your first project.

First, install the dx-www CLI using cargo:

cargo install dx-www-cli

Then create a new project:

dx new my-app
cd my-app

Your project structure will look like this:

my-app/
├── app/
│   ├── layout.tsx
│   └── page.tsx
├── components/
├── lib/
│   └── stores/
├── public/
├── styles/
│   └── globals.css
└── dx

Now you can start the development server:

dx dev

Open http://localhost:3000 to see your app running!`,
        date: '2026-01-08',
        author: 'DX Team',
        tags: ['tutorial', 'getting-started', 'dx-www'],
    },
    {
        slug: 'understanding-dx-route-packets',
        title: 'Understanding DX Route Packets',
        excerpt: 'A deep dive into the source-owned route packets and receipts that power dx-www.',
        content: `DX-WWW uses source-owned route packets and receipts to keep public output tiny while preserving proof data outside deployable runtime bytes. Static routes can ship meaningful HTML and CSS with no browser JavaScript, while interactive routes opt into the smallest compiler-owned runtime they need.

The route packet model consists of:

1. Route unit: compiled HTML, metadata, state graph, and proof surfaces
2. Public output: deployable HTML, CSS, assets, and route-local runtime only when needed
3. Evidence bundle: .dx receipts, serializer contracts, source maps, and replay commands

This approach has several advantages:

- Smaller payload size (typically 50-70% smaller than HTML)
- Faster startup for no-JS and micro-JS routes
- Honest proof receipts for build, check, style, icons, and runtime claims

Simple static routes need no browser runtime at all. Interactive routes use compiler-owned event, state, and island runtimes only when the source requires them.`,
        date: '2026-01-05',
        author: 'DX Team',
        tags: ['architecture', 'route-packets', 'performance'],
    },
    {
        slug: 'server-side-rendering-guide',
        title: 'Server-Side Rendering with dx-www',
        excerpt: 'How to implement SSR for better SEO and initial load performance.',
        content: `Server-side rendering (SSR) is essential for SEO and fast initial page loads. dx-www makes SSR simple and efficient.

To enable server rendering, keep the extensionless dx config at the project root and use App Router files:

app/page.tsx
app/layout.tsx
app/api/hello/route.ts

Then create a server entry point:

export const metadata = {
    title: 'Server-rendered DX route',
};

export default function Page() {
    return <main>Server-rendered HTML with optional DX-native interactivity.</main>;
}

The framework renders meaningful HTML first, then emits no JavaScript, micro JavaScript, or island JavaScript according to what the route actually uses.

DX-WWW keeps this source-owned and receipt-backed, so server output, no-JS fallback, and runtime boundaries stay inspectable.`,
        date: '2026-01-02',
        author: 'DX Team',
        tags: ['ssr', 'seo', 'performance'],
    },
];
