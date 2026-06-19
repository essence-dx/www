import { bearerGet, value } from "./request-utils";
import type { WorldConnectionProvider } from "./types";

export const commerceConnectionProviders = [
  {
    id: "stripe",
    categoryId: "payments",
    name: "Stripe",
    packageId: "payments/stripe",
    requiredEnv: ["STRIPE_SECRET_KEY"],
    optionalEnv: ["STRIPE_WEBHOOK_SECRET", "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY"],
    receiptSchema: "dx.forge.world.payments",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://api.stripe.com/v1/balance",
      buildRequest: async (env) => bearerGet("https://api.stripe.com/v1/balance", value(env, "STRIPE_SECRET_KEY")),
    },
    nextAction: "Import a redacted Stripe balance readiness receipt before enabling checkout or webhook live claims.",
  },
  {
    id: "lemon-squeezy",
    categoryId: "payments",
    name: "Lemon Squeezy",
    packageId: "payments/lemon-squeezy",
    requiredEnv: ["LEMON_SQUEEZY_API_KEY"],
    optionalEnv: ["LEMON_SQUEEZY_WEBHOOK_SECRET", "LEMON_SQUEEZY_STORE_ID"],
    receiptSchema: "dx.forge.world.payments",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://api.lemonsqueezy.com/v1/stores",
      buildRequest: async (env) =>
        bearerGet("https://api.lemonsqueezy.com/v1/stores", value(env, "LEMON_SQUEEZY_API_KEY"), {
          "Content-Type": "application/vnd.api+json",
        }),
    },
    nextAction: "Import a redacted Lemon Squeezy stores readiness receipt before enabling checkout links or webhooks.",
  },
  {
    id: "paddle",
    categoryId: "payments",
    name: "Paddle",
    packageId: "payments/paddle",
    requiredEnv: ["PADDLE_API_KEY"],
    optionalEnv: ["PADDLE_WEBHOOK_SECRET", "PADDLE_ENVIRONMENT"],
    receiptSchema: "dx.forge.world.payments",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://api.paddle.com/products?per_page=1",
      buildRequest: async (env) => {
        const host = value(env, "PADDLE_ENVIRONMENT") === "sandbox" ? "sandbox-api.paddle.com" : "api.paddle.com";
        return bearerGet(`https://${host}/products?per_page=1`, value(env, "PADDLE_API_KEY"));
      },
    },
    nextAction: "Import a redacted Paddle product-catalog readiness receipt before enabling merchant-of-record flows.",
  },
] satisfies readonly WorldConnectionProvider[];
