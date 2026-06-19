import "./globals.css";

export const metadata = {
  title: "Next Fair Counter",
  description: "Minimal interactive counter for fair framework payload testing.",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
