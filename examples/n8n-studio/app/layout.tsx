import "../styles/globals.css";

type RootLayoutProps = {
  children: any;
};

export const metadata = {
  title: "DX WWW n8n Studio",
  description: "Source-owned n8n-compatible automation studio for the DX WWW ecosystem.",
} as const;

export default function RootLayout({ children }: RootLayoutProps) {
  return (
    <html lang="en">
      <head>
        <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </head>
      <body>{children}</body>
    </html>
  );
}

