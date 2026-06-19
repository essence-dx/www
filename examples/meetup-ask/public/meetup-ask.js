function setupMeetupAsk() {
  const escapePhrases = [
    "Nope! 🏃",
    "Run away!",
    "Still no!",
    "Catch me 😜",
    "Nuh uh! 😅",
    "Noooo~",
    "Can't click!",
    "Too slow! 💨",
    "Almost! 😆",
    "Dodge! 🌀",
    "Nice try 😏",
    "Hehe nope!",
    "Sneaky! 😜",
    "Zoom zoom!",
    "Not today!",
    "Ha! Missed!",
    "Keep trying!",
    "Almost there…",
    "One more…",
    "Last escape! 😂",
  ];
  const confettiColors = [
    "#D14F6C",
    "#F2B8C2",
    "#FFD700",
    "#9FE1CB",
    "#534AB7",
    "#FFAA55",
  ];

  const mainCard = document.getElementById("mainCard");
  const successCard = document.getElementById("successCard");
  const yesButton = document.getElementById("yesBtn");
  const noButton = document.getElementById("noBtn");
  const jumpZone = document.getElementById("jumpZone");
  const emailForm = document.getElementById("emailForm");
  const emailInput = document.getElementById("replyEmail");
  const nicknameInput = document.getElementById("nickname");
  const submitButton = document.getElementById("submitBtn");
  const statusText = document.getElementById("statusText");
  const resultTitle = document.getElementById("resultTitle");
  const resultMessage = document.getElementById("resultMessage");

  if (
    !mainCard ||
    !successCard ||
    !yesButton ||
    !noButton ||
    !jumpZone ||
    !emailForm ||
    !emailInput ||
    !nicknameInput ||
    !submitButton ||
    !statusText ||
    !resultTitle ||
    !resultMessage
  ) {
    return;
  }

  let escapes = 0;
  let pending = false;

  function setPending(nextPending) {
    pending = nextPending;
    yesButton.disabled = nextPending;
    submitButton.disabled = nextPending;
  }

  function showEmailForm() {
    emailForm.hidden = false;
    statusText.textContent = "Your email only goes into this one reply.";
    window.setTimeout(() => {
      emailInput.focus();
    }, 0);
  }

  function setResultMessage(lines) {
    resultMessage.replaceChildren();
    lines.forEach((line, index) => {
      if (index > 0) {
        resultMessage.appendChild(document.createElement("br"));
        resultMessage.appendChild(document.createElement("br"));
      }
      resultMessage.appendChild(document.createTextNode(line));
    });
  }

  function showResult(title, messageLines) {
    mainCard.style.display = "none";
    resultTitle.textContent = title;
    setResultMessage(messageLines);
    successCard.style.display = "block";
    successCard.focus({ preventScroll: true });
  }

  function burstConfetti() {
    const confetti = document.createElement("div");
    confetti.className = "meetup-confetti";
    document.body.appendChild(confetti);

    for (let index = 0; index < 90; index += 1) {
      const bit = document.createElement("div");
      const angle = Math.random() * Math.PI * 2;
      const distance = 80 + Math.random() * 320;

      bit.className = "meetup-confetti-bit";
      bit.style.background = confettiColors[index % confettiColors.length];
      bit.style.left = "50%";
      bit.style.top = "50%";
      bit.style.setProperty("--dx", `${Math.cos(angle) * distance}px`);
      bit.style.setProperty("--dy", `${Math.sin(angle) * distance}px`);
      bit.style.animationDelay = `${Math.random() * 0.2}s`;
      confetti.appendChild(bit);
    }

    window.setTimeout(() => {
      confetti.remove();
    }, 2000);
  }

  function moveNoButton() {
    if (pending || noButton.style.display === "none") {
      return;
    }

    noButton.textContent = escapePhrases[escapes % escapePhrases.length];
    escapes += 1;

    const zoneWidth = jumpZone.offsetWidth;
    const zoneHeight = jumpZone.offsetHeight;
    const buttonWidth = noButton.offsetWidth || 130;
    const buttonHeight = noButton.offsetHeight || 40;
    const currentLeft =
      Number.parseFloat(noButton.style.left) || zoneWidth / 2 - buttonWidth / 2;
    const currentTop =
      Number.parseFloat(noButton.style.top) || zoneHeight / 2 - buttonHeight / 2;
    const maxLeft = Math.max(8, zoneWidth - buttonWidth - 16);
    const maxTop = Math.max(8, zoneHeight - buttonHeight - 16);
    let nextLeft = 0;
    let nextTop = 0;
    let tries = 0;

    do {
      nextLeft = 8 + Math.random() * maxLeft;
      nextTop = 8 + Math.random() * maxTop;
      tries += 1;
    } while (
      tries < 30 &&
      Math.abs(nextLeft - currentLeft) < 60 &&
      Math.abs(nextTop - currentTop) < 30
    );

    noButton.style.transform = "none";
    noButton.style.left = `${nextLeft}px`;
    noButton.style.top = `${nextTop}px`;
  }

  async function sendResponse(payload) {
    if (pending) {
      return;
    }

    setPending(true);
    statusText.textContent = "Sending...";

    try {
      const response = await fetch("/api/meetup-response", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
      });
      const result = await response.json().catch(() => ({}));

      if (!response.ok || !result.ok) {
        throw new Error(result.message || "Could not send the message.");
      }

      burstConfetti();
      showResult("You said yes!! 💗", [
        "That honestly made me so happy!",
        "I'll reach out soon and we can figure out the details. Can't wait! ✨",
      ]);
    } catch (error) {
      statusText.textContent =
        error instanceof Error
          ? error.message
          : "Could not send the message right now.";
      setPending(false);
    }
  }

  yesButton.addEventListener("click", showEmailForm);
  noButton.addEventListener("click", moveNoButton);
  emailForm.addEventListener("submit", (event) => {
    event.preventDefault();
    sendResponse({
      choice: "yes",
      email: emailInput.value,
      nickname: nicknameInput.value,
    });
  });
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", setupMeetupAsk, { once: true });
} else {
  setupMeetupAsk();
}
