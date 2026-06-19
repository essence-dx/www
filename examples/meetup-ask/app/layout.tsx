import "../styles/globals.css";

type RootLayoutProps = {
  children: any;
};

export default function RootLayout({ children }: RootLayoutProps) {
  return (
    <html lang="en">
      <head>
        <title>Want to hang out?</title>
        <meta
          name="description"
          content="A soft DX WWW meetup invitation."
        />
        <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
      </head>
      <body>{children}</body>
    </html>
  );
}
