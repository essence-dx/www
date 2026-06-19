const BREVO_URL = "https://api.brevo.com/v3/smtp/email";
const FAMILY_EMAIL = "ajju40959@gmail.com";

function json(response, status, body) {
  response.statusCode = status;
  response.setHeader("Content-Type", "application/json");
  response.end(JSON.stringify(body));
}

async function readJsonBody(request) {
  if (request.body && typeof request.body === "object") {
    return request.body;
  }
  if (typeof request.body === "string") {
    return JSON.parse(request.body || "{}");
  }

  const chunks = [];
  for await (const chunk of request) {
    chunks.push(Buffer.from(chunk));
  }
  const raw = Buffer.concat(chunks).toString("utf8").trim();
  return raw ? JSON.parse(raw) : {};
}

function cleanEmail(value) {
  const email = String(value || "").trim().toLowerCase();
  if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email) || email.length > 254) {
    return null;
  }
  return email;
}

function htmlEscape(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function letterHtml() {
  return `
    <p>Hi Rivo,</p>
    <p>I know things may not feel simple anymore, and I do not want to force anything.</p>
    <p>I just wanted to say this honestly: I still care, and I would be grateful for one calm conversation if you are open to it.</p>
    <p>If studying and life are already heavy, I understand. If you can give me one chance to talk, message me on social media or reply here.</p>
    <p>No pressure. Just one honest request from Emon.</p>
  `;
}

function letterText() {
  return [
    "Hi Rivo,",
    "",
    "I know things may not feel simple anymore, and I do not want to force anything.",
    "I just wanted to say this honestly: I still care, and I would be grateful for one calm conversation if you are open to it.",
    "If studying and life are already heavy, I understand. If you can give me one chance to talk, message me on social media or reply here.",
    "",
    "No pressure. Just one honest request from Emon.",
  ].join("\n");
}

async function sendBrevoEmail(message) {
  const apiKey = process.env.BREVO_API_KEY;
  if (!apiKey) {
    throw new Error("BREVO_API_KEY is not configured.");
  }

  const senderEmail = process.env.BREVO_SENDER_EMAIL || FAMILY_EMAIL;
  const senderName = process.env.BREVO_SENDER_NAME || "Emon";

  const response = await fetch(BREVO_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "api-key": apiKey,
    },
    body: JSON.stringify({
      sender: {
        email: senderEmail,
        name: senderName,
      },
      ...message,
    }),
  });

  if (!response.ok) {
    const detail = await response.text();
    throw new Error(`Brevo rejected the email: ${detail.slice(0, 240)}`);
  }
}

async function sendYesEmails(replyEmail) {
  await sendBrevoEmail({
    to: [{ email: FAMILY_EMAIL, name: "Family inbox" }],
    subject: "Meetup ask response: yes",
    htmlContent: `<p>She said yes and shared this email:</p><p><strong>${htmlEscape(
      replyEmail
    )}</strong></p>`,
    textContent: `She said yes and shared this email: ${replyEmail}`,
  });

  await sendBrevoEmail({
    to: [{ email: replyEmail }],
    subject: "Can we talk once?",
    htmlContent: letterHtml(),
    textContent: letterText(),
  });
}

async function sendNoEmail() {
  await sendBrevoEmail({
    to: [{ email: FAMILY_EMAIL, name: "Family inbox" }],
    subject: "Meetup ask response: no",
    htmlContent:
      "<p>She clicked no on the meetup page. Please respect the answer and give her space.</p>",
    textContent:
      "She clicked no on the meetup page. Please respect the answer and give her space.",
  });
}

module.exports = async function handler(request, response) {
  if (request.method !== "POST") {
    return json(response, 405, { ok: false, message: "Use POST." });
  }

  try {
    const body = await readJsonBody(request);
    if (body.nickname) {
      return json(response, 200, { ok: true });
    }

    const choice = String(body.choice || "").trim().toLowerCase();
    if (choice === "yes") {
      const replyEmail = cleanEmail(body.email);
      if (!replyEmail) {
        return json(response, 400, {
          ok: false,
          message: "Please enter a valid email address.",
        });
      }
      await sendYesEmails(replyEmail);
      return json(response, 200, { ok: true });
    }

    if (choice === "no") {
      await sendNoEmail();
      return json(response, 200, { ok: true });
    }

    return json(response, 400, { ok: false, message: "Choose yes or no." });
  } catch (error) {
    console.error("meetup-response failed", error);
    return json(response, 500, {
      ok: false,
      message: "The message could not be sent right now.",
    });
  }
};
