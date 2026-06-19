import "../styles/globals.css";

type RootLayoutProps = {
  children: any;
};

export const metadata = {
  title: "DX WWW Whiteboard",
  description: "Source-owned whiteboard workspace for the DX WWW ecosystem.",
} as const;

export default function RootLayout({ children }: RootLayoutProps) {
  return (
    <html lang="en">
      <head>
        <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
        <link rel="apple-touch-icon" href="/icon.svg" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </head>
      <body>{children}</body>
    </html>
  );
}
