export function docSections() {
  return Array.from({ length: 160 }, (_, index) => {
    const number = index + 1;
    const principle = ["routing", "serialization", "styling", "deployment"][number % 4];
    return {
      number,
      principle,
      copy:
        "This block covers cache behavior, payload shape, routing boundaries, and production maintenance for the same generated content.",
    };
  });
}

export function cards() {
  const categories = ["auth", "dashboard", "commerce", "editor", "analytics"];
  return Array.from({ length: 180 }, (_, index) => {
    const number = index + 1;
    const category = categories[number % categories.length];
    return {
      number,
      category,
      title: `${category} component ${number}`,
      copy:
        "Editable source-owned component packaging with predictable upgrade metadata.",
    };
  });
}

export function dashboardRows() {
  const plans = ["Enterprise", "Pro", "Team", "Starter"];
  const regions = ["APAC", "EU", "NA", "LATAM", "MEA"];
  return Array.from({ length: 1200 }, (_, index) => {
    const number = index + 1;
    const status = number % 9 === 0 ? "Review" : "Healthy";
    const plan = plans[number % plans.length];
    const region = regions[number % regions.length];
    return {
      number,
      account: `Account ${number}`,
      plan,
      region,
      status,
      mrr: 400 + (number % 37) * 91,
      risk: (number * 7) % 100,
      search: `account ${number} ${plan} ${region} ${status}`.toLowerCase(),
    };
  });
}
