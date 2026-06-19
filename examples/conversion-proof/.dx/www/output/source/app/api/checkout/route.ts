export async function POST(request: Request) {
  const body = await request.json();
  return {
    ok: true,
    status: 202,
    mode: "safe-stripe-contract",
    plan: body.plan,
    quantity: body.quantity,
    appOwnedBoundary: "Create a real Stripe Checkout Session after STRIPE_SECRET_KEY is configured.",
  };
}
