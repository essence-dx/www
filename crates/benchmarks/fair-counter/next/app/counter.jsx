"use client";

import { useState } from "react";

export function Counter() {
  const [count, setCount] = useState(0);

  return (
    <>
      <div className="counter" aria-live="polite" aria-atomic="true">
        {count}
      </div>
      <div role="group" aria-label="Counter controls">
        <button onClick={() => setCount((value) => value + 1)} type="button">
          Increment
        </button>
        <button onClick={() => setCount((value) => value - 1)} type="button">
          Decrement
        </button>
        <button onClick={() => setCount(0)} type="button">
          Reset
        </button>
      </div>
    </>
  );
}
