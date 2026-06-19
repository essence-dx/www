import "../styles/globals.css";
import "../styles/charts.css";
import { chartFaviconPath, chartRuntimePath, chartTouchIconPath } from "../lib/charts";

type RootLayoutProps = {
  children: any;
};

export default function RootLayout({ children }: RootLayoutProps) {
  return (
    <html lang="en">
      <head>
        <link rel="icon" href={chartFaviconPath} type="image/svg+xml" />
        <link rel="apple-touch-icon" href={chartTouchIconPath} />
        <script src={chartRuntimePath} defer />
      </head>
      <body>{children}</body>
    </html>
  );
}
