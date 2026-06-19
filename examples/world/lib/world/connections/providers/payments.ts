import type { WorldConnectionProbe } from "../contracts";
import { configuredReadinessProbe, httpProbe } from "./shared";

function stripeBasicAuth(secret: string): string {
  return `Basic ${btoa(`${secret}:`)}`;
}

export const paymentConnectionProbes: readonly WorldConnectionProbe[] = [
  httpProbe(
    {
      id: "stripe-account",
      providerId: "stripe",
      packageId: "commerce/stripe",
      name: "Stripe account",
      category: "Payments",
      kind: "http",
      endpoint: "https://api.stripe.com/v1/account",
      documentationUrl: "https://docs.stripe.com/api/accounts/retrieve",
      requiredEnv: ["STRIPE_SECRET_KEY"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: "https://api.stripe.com/v1/account",
      headers: {
        Authorization: stripeBasicAuth(env.STRIPE_SECRET_KEY ?? ""),
      },
      expectedStatuses: [200],
      evidence: "account-readable",
    }),
  ),
  configuredReadinessProbe(
    {
      id: "lemon-squeezy-env-readiness",
      providerId: "lemon-squeezy",
      packageId: "commerce/lemon-squeezy",
      name: "Lemon Squeezy",
      category: "Payments",
      kind: "env",
      endpoint: "env:LEMON_SQUEEZY_API_KEY",
      documentationUrl: "https://docs.lemonsqueezy.com/api",
      requiredEnv: ["LEMON_SQUEEZY_API_KEY"],
      optionalEnv: ["LEMON_SQUEEZY_STORE_ID"],
    },
    "Lemon Squeezy env is present; checkout/product proof should run through a Forge payment adapter.",
  ),
  configuredReadinessProbe(
    {
      id: "paddle-env-readiness",
      providerId: "paddle",
      packageId: "commerce/paddle",
      name: "Paddle",
      category: "Payments",
      kind: "env",
      endpoint: "env:PADDLE_API_KEY",
      documentationUrl: "https://developer.paddle.com/api-reference/overview",
      requiredEnv: ["PADDLE_API_KEY"],
      optionalEnv: ["PADDLE_ENVIRONMENT"],
    },
    "Paddle env is present; catalog and checkout validation should run through a Forge payment adapter.",
  ),
];
