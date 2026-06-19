export const metadata = {
  title: "Want to hang out?",
  description: "A soft DX WWW meetup invitation.",
} as const;

export default function MeetupAskPage() {
  return (
    <main className="meetup-page" data-dx-route="/">
      <section className="meetup-shell" aria-label="Meetup invitation">
        <article className="meetup-card" id="mainCard">
          <span className="meetup-heart" aria-hidden="true">
            💝
          </span>
          <h1>
            Want to hang out
            <br />
            {"with me? "}
            <em>Just us</em>
            {" 🌸"}
          </h1>
          <p className="meetup-sub">
            No pressure, no labels — just a fun casual meetup ✨
          </p>
          <div className="meetup-divider" aria-hidden="true"></div>
          <div className="meetup-message">
            <p>
              "I've been meaning to ask... would you like to spend some time
              together? Just the two of us, relaxed and easy. I think it'd be
              really nice 🙂"
            </p>
          </div>
          <p className="meetup-question">So what do you think?</p>
          <button className="meetup-yes-button" id="yesBtn" type="button">
            😊 Yes, sounds fun!
          </button>
          <div className="meetup-action-gap" aria-hidden="true"></div>
          <div className="meetup-jump-zone" id="jumpZone">
            <button className="meetup-no-button" id="noBtn" type="button">
              Hmm, no...
            </button>
            <span className="meetup-zone-label">try clicking "no" 😏</span>
          </div>
          <form className="meetup-email-form" id="emailForm" hidden>
            <label className="meetup-email-label" htmlFor="replyEmail">
              Where should I send the message?
            </label>
            <input
              className="meetup-email-input"
              id="replyEmail"
              name="email"
              type="email"
              autoComplete="email"
              placeholder="your email"
              required
            />
            <input
              className="meetup-hidden-field"
              id="nickname"
              name="nickname"
              type="text"
              tabIndex={-1}
              autoComplete="off"
            />
            <button className="meetup-submit-button" id="submitBtn" type="submit">
              Send my yes
            </button>
          </form>
          <p className="meetup-status" id="statusText" aria-live="polite"></p>
        </article>

        <article
          className="meetup-success"
          id="successCard"
          aria-live="polite"
          tabIndex={-1}
        >
          <span className="meetup-success-emoji" aria-hidden="true">
            🎉
          </span>
          <h2 id="resultTitle">You said yes!! 💗</h2>
          <p id="resultMessage">
            That honestly made me so happy!
            <br />
            <br />
            I'll reach out soon and we can figure out the details. Can't wait!
            ✨
          </p>
        </article>
      </section>
      <script src="/public/meetup-ask.js" defer></script>
    </main>
  );
}
