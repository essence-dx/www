//! Drizzle ORM source templates for the Forge registry.

pub(super) const DRIZZLE_SQLITE_VERSION: &str = "0.1.0";

pub(super) fn drizzle_sqlite_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/db/drizzle/client.ts", DRIZZLE_CLIENT_TS),
        ("js/db/drizzle/schema.ts", DRIZZLE_SCHEMA_TS),
        ("js/db/drizzle/views.ts", DRIZZLE_VIEWS_TS),
        ("js/db/drizzle/queries.ts", DRIZZLE_QUERIES_TS),
        ("js/db/drizzle/migrations.ts", DRIZZLE_MIGRATIONS_TS),
        (
            "js/db/drizzle/relational-queries.ts",
            DRIZZLE_RELATIONAL_QUERIES_TS,
        ),
        ("js/db/drizzle/joins.ts", DRIZZLE_JOINS_TS),
        ("js/db/drizzle/set-operations.ts", DRIZZLE_SET_OPERATIONS_TS),
        ("js/db/drizzle/cte-queries.ts", DRIZZLE_CTE_QUERIES_TS),
        ("js/db/drizzle/transactions.ts", DRIZZLE_TRANSACTIONS_TS),
        (
            "js/db/drizzle/prepared-queries.ts",
            DRIZZLE_PREPARED_QUERIES_TS,
        ),
        ("js/db/drizzle/upserts.ts", DRIZZLE_UPSERTS_TS),
        ("js/db/drizzle/mutations.ts", DRIZZLE_MUTATIONS_TS),
        ("js/db/drizzle/analytics.ts", DRIZZLE_ANALYTICS_TS),
        ("js/db/drizzle/replicas.ts", DRIZZLE_REPLICAS_TS),
        (
            "js/db/drizzle/dashboard-workflow.ts",
            DRIZZLE_DASHBOARD_WORKFLOW_TS,
        ),
        ("js/db/drizzle/metadata.ts", DRIZZLE_METADATA_TS),
        ("js/db/drizzle/README.md", DRIZZLE_README_MD),
    ]
}

const DRIZZLE_CLIENT_TS: &str = r#"import Database from "better-sqlite3";
import { drizzle, type BetterSQLite3Database } from "drizzle-orm/better-sqlite3";

import * as schema from "./schema";

export type DxDrizzleDatabase = BetterSQLite3Database<typeof schema>;

export type DxDrizzleConnection = {
  db: DxDrizzleDatabase;
  sqlite: Database.Database;
};

export function createDxDrizzleConnection(
  path = process.env.DATABASE_URL ?? "sqlite.db",
): DxDrizzleConnection {
  const sqlite = new Database(path);
  const db = drizzle(sqlite, { schema });

  return { db, sqlite };
}
"#;

const DRIZZLE_SCHEMA_TS: &str = r#"import { relations, type InferInsertModel, type InferSelectModel } from "drizzle-orm";
import { index, integer, sqliteTable, text, uniqueIndex } from "drizzle-orm/sqlite-core";

export const users = sqliteTable(
  "users",
  {
    id: integer("id").primaryKey({ autoIncrement: true }),
    email: text("email").notNull(),
    name: text("name").notNull(),
    role: text("role", { enum: ["admin", "member"] }).notNull().default("member"),
    createdAt: integer("created_at", { mode: "timestamp_ms" })
      .notNull()
      .$defaultFn(() => new Date()),
    updatedAt: integer("updated_at", { mode: "timestamp_ms" })
      .notNull()
      .$defaultFn(() => new Date()),
  },
  (table) => [uniqueIndex("users_email_idx").on(table.email), index("users_role_idx").on(table.role)],
);

export const posts = sqliteTable(
  "posts",
  {
    id: integer("id").primaryKey({ autoIncrement: true }),
    authorId: integer("author_id")
      .notNull()
      .references(() => users.id, { onDelete: "cascade" }),
    slug: text("slug").notNull(),
    title: text("title").notNull(),
    body: text("body").notNull(),
    status: text("status", { enum: ["draft", "published", "archived"] }).notNull().default("draft"),
    publishedAt: integer("published_at", { mode: "timestamp_ms" }),
    createdAt: integer("created_at", { mode: "timestamp_ms" })
      .notNull()
      .$defaultFn(() => new Date()),
    updatedAt: integer("updated_at", { mode: "timestamp_ms" })
      .notNull()
      .$defaultFn(() => new Date()),
  },
  (table) => [
    uniqueIndex("posts_slug_idx").on(table.slug),
    index("posts_author_status_idx").on(table.authorId, table.status),
  ],
);

export const usersRelations = relations(users, ({ many }) => ({
  posts: many(posts),
}));

export const postsRelations = relations(posts, ({ one }) => ({
  author: one(users, {
    fields: [posts.authorId],
    references: [users.id],
  }),
}));

export type User = InferSelectModel<typeof users>;
export type NewUser = InferInsertModel<typeof users>;
export type Post = InferSelectModel<typeof posts>;
export type NewPost = InferInsertModel<typeof posts>;
"#;

const DRIZZLE_VIEWS_TS: &str = r#"import { desc, eq, getViewSelectedFields, sql } from "drizzle-orm";
import { integer, sqliteView, text } from "drizzle-orm/sqlite-core";

import type { DxDrizzleDatabase } from "./client";
import { posts, users } from "./schema";

export const publishedPostSummaries = sqliteView("published_post_summaries").as((qb) =>
  qb
    .select({
      id: posts.id,
      slug: posts.slug,
      title: posts.title,
      authorName: users.name,
      publishedAt: posts.publishedAt,
      bodyLength: sql<number>`length(${posts.body})`.as("body_length"),
    })
    .from(posts)
    .innerJoin(users, eq(posts.authorId, users.id))
    .where(eq(posts.status, "published")),
);

export const existingPublishedPostSummaries = sqliteView("existing_published_post_summaries", {
  id: integer("id").primaryKey(),
  slug: text("slug").notNull(),
  title: text("title").notNull(),
  authorName: text("author_name").notNull(),
}).existing();

export type PublishedPostSummary = typeof publishedPostSummaries.$inferSelect;

export function listPublishedPostSummaries(
  db: DxDrizzleDatabase,
  limit = 20,
): PublishedPostSummary[] {
  return db
    .select()
    .from(publishedPostSummaries)
    .orderBy(desc(publishedPostSummaries.publishedAt))
    .limit(limit)
    .all();
}

export function readPublishedPostSummaryFields(): string[] {
  return Object.keys(getViewSelectedFields(publishedPostSummaries));
}
"#;

const DRIZZLE_QUERIES_TS: &str = r#"import { and, desc, eq, sql } from "drizzle-orm";

import type { DxDrizzleDatabase } from "./client";
import { posts, users, type NewPost, type NewUser } from "./schema";

export type ListUsersInput = {
  role?: "admin" | "member";
  search?: string;
  limit?: number;
};

export function createUser(db: DxDrizzleDatabase, input: NewUser) {
  return db.insert(users).values(input).returning().get();
}

export function findUserByEmail(db: DxDrizzleDatabase, email: string) {
  return db.select().from(users).where(eq(users.email, email)).get();
}

export function listUsers(db: DxDrizzleDatabase, input: ListUsersInput = {}) {
  const search = input.search?.trim().toLowerCase();
  const searchClause = search
    ? sql`lower(${users.email}) like ${`%${search}%`} or lower(${users.name}) like ${`%${search}%`}`
    : undefined;
  const roleClause = input.role ? eq(users.role, input.role) : undefined;
  const where = roleClause && searchClause ? and(roleClause, searchClause) : roleClause ?? searchClause;

  return db
    .select()
    .from(users)
    .where(where)
    .orderBy(desc(users.createdAt))
    .limit(input.limit ?? 25)
    .all();
}

export function createPost(db: DxDrizzleDatabase, input: NewPost) {
  return db.insert(posts).values(input).returning().get();
}

export function listPublishedPosts(db: DxDrizzleDatabase, authorId?: number) {
  const published = eq(posts.status, "published");
  const where = authorId === undefined ? published : and(published, eq(posts.authorId, authorId));

  return db
    .select({
      id: posts.id,
      slug: posts.slug,
      title: posts.title,
      authorId: posts.authorId,
      publishedAt: posts.publishedAt,
      bodyLength: sql<number>`length(${posts.body})`,
    })
    .from(posts)
    .where(where)
    .orderBy(desc(posts.publishedAt))
    .limit(20)
    .all();
}
"#;

const DRIZZLE_MIGRATIONS_TS: &str = r#"import { migrate } from "drizzle-orm/better-sqlite3/migrator";
import type { MigrationConfig } from "drizzle-orm/migrator";

import type { DxDrizzleDatabase } from "./client";

export type DxDrizzleMigrationOptions = Partial<
  Pick<MigrationConfig, "migrationsFolder" | "migrationsTable" | "migrationsSchema">
>;

export const dxDrizzleMigrationDefaults = {
  migrationsFolder: "./drizzle",
} satisfies Pick<MigrationConfig, "migrationsFolder">;

export function buildDxDrizzleMigrationConfig(
  options: DxDrizzleMigrationOptions = {},
): MigrationConfig {
  const config: MigrationConfig = {
    migrationsFolder: options.migrationsFolder ?? dxDrizzleMigrationDefaults.migrationsFolder,
  };

  if (options.migrationsTable) {
    config.migrationsTable = options.migrationsTable;
  }

  if (options.migrationsSchema) {
    config.migrationsSchema = options.migrationsSchema;
  }

  return config;
}

export function applyDxDrizzleMigrations(
  db: DxDrizzleDatabase,
  options: DxDrizzleMigrationOptions = {},
): MigrationConfig {
  const config = buildDxDrizzleMigrationConfig(options);
  migrate(db, config);
  return config;
}
"#;

const DRIZZLE_RELATIONAL_QUERIES_TS: &str = r#"import { eq } from "drizzle-orm";

import type { DxDrizzleDatabase } from "./client";
import { posts, users } from "./schema";

export type ListUsersWithPostsInput = {
  role?: "admin" | "member";
  limit?: number;
};

export function listUsersWithPosts(
  db: DxDrizzleDatabase,
  input: ListUsersWithPostsInput = {},
) {
  return db.query.users.findMany({
    where: input.role ? eq(users.role, input.role) : undefined,
    with: {
      posts: true,
    },
    limit: input.limit ?? 25,
  });
}

export function findPostWithAuthor(db: DxDrizzleDatabase, slug: string) {
  return db.query.posts.findFirst({
    where: eq(posts.slug, slug),
    with: {
      author: true,
    },
  });
}
"#;

const DRIZZLE_JOINS_TS: &str = r#"import { desc, eq, sql } from "drizzle-orm";

import type { DxDrizzleDatabase } from "./client";
import { posts, users } from "./schema";

export type PublishedPostPreview = {
  id: number;
  slug: string;
  title: string;
  authorName: string;
  authorEmail: string;
  bodyLength: number;
};

export type UserPostRow = {
  userId: number;
  email: string;
  name: string;
  postId: number | null;
  postSlug: string | null;
  postStatus: "draft" | "published" | "archived" | null;
};

export function listPublishedPostPreviews(
  db: DxDrizzleDatabase,
  limit = 20,
): PublishedPostPreview[] {
  return db
    .select({
      id: posts.id,
      slug: posts.slug,
      title: posts.title,
      authorName: users.name,
      authorEmail: users.email,
      bodyLength: sql<number>`length(${posts.body})`,
    })
    .from(posts)
    .innerJoin(users, eq(posts.authorId, users.id))
    .where(eq(posts.status, "published"))
    .orderBy(desc(posts.publishedAt))
    .limit(limit)
    .all();
}

export function listUsersWithOptionalPosts(
  db: DxDrizzleDatabase,
  limit = 25,
): UserPostRow[] {
  return db
    .select({
      userId: users.id,
      email: users.email,
      name: users.name,
      postId: posts.id,
      postSlug: posts.slug,
      postStatus: posts.status,
    })
    .from(users)
    .leftJoin(posts, eq(users.id, posts.authorId))
    .orderBy(desc(users.createdAt))
    .limit(limit)
    .all();
}
"#;

const DRIZZLE_SET_OPERATIONS_TS: &str = r#"import { asc, eq, sql } from "drizzle-orm";
import { except, intersect, union, unionAll } from "drizzle-orm/sqlite-core";

import type { DxDrizzleDatabase } from "./client";
import { posts, users } from "./schema";

export type LaunchAudienceSource = "admin" | "published-author";

export type LaunchAudienceRow = {
  id: number;
  label: string;
  source: LaunchAudienceSource;
};

export type PublicationCandidateStage = "published" | "draft";

export type PublicationCandidateRow = {
  id: number;
  slug: string;
  title: string;
  stage: PublicationCandidateStage;
};

export type AuthorPublishPermissionRow = {
  id: number;
  email: string;
};

export type PostIdentityRow = {
  id: number;
  slug: string;
  title: string;
};

function selectAdminAudience(db: DxDrizzleDatabase) {
  return db
    .select({
      id: users.id,
      label: users.email,
      source: sql<LaunchAudienceSource>`'admin'`,
    })
    .from(users)
    .where(eq(users.role, "admin"));
}

function selectPublishedAuthorAudience(db: DxDrizzleDatabase) {
  return db
    .select({
      id: users.id,
      label: users.email,
      source: sql<LaunchAudienceSource>`'published-author'`,
    })
    .from(users)
    .innerJoin(posts, eq(posts.authorId, users.id))
    .where(eq(posts.status, "published"));
}

function selectPublishedPostCandidates(db: DxDrizzleDatabase) {
  return db
    .select({
      id: posts.id,
      slug: posts.slug,
      title: posts.title,
      stage: sql<PublicationCandidateStage>`'published'`,
    })
    .from(posts)
    .where(eq(posts.status, "published"));
}

function selectDraftPostCandidates(db: DxDrizzleDatabase) {
  return db
    .select({
      id: posts.id,
      slug: posts.slug,
      title: posts.title,
      stage: sql<PublicationCandidateStage>`'draft'`,
    })
    .from(posts)
    .where(eq(posts.status, "draft"));
}

function selectAuthorsWithPublishedPosts(db: DxDrizzleDatabase) {
  return db
    .select({
      id: users.id,
      email: users.email,
    })
    .from(users)
    .innerJoin(posts, eq(posts.authorId, users.id))
    .where(eq(posts.status, "published"));
}

function selectAdminAuthors(db: DxDrizzleDatabase) {
  return db
    .select({
      id: users.id,
      email: users.email,
    })
    .from(users)
    .where(eq(users.role, "admin"));
}

function selectAllPostIdentities(db: DxDrizzleDatabase) {
  return db
    .select({
      id: posts.id,
      slug: posts.slug,
      title: posts.title,
    })
    .from(posts);
}

function selectPublishedPostIdentities(db: DxDrizzleDatabase) {
  return db
    .select({
      id: posts.id,
      slug: posts.slug,
      title: posts.title,
    })
    .from(posts)
    .where(eq(posts.status, "published"));
}

export function listLaunchAudience(
  db: DxDrizzleDatabase,
  limit = 50,
): LaunchAudienceRow[] {
  return union(
    selectAdminAudience(db),
    selectPublishedAuthorAudience(db),
  )
    .orderBy(asc(sql`label`))
    .limit(limit)
    .all();
}

export function listPublicationCandidates(
  db: DxDrizzleDatabase,
  limit = 50,
): PublicationCandidateRow[] {
  return unionAll(
    selectPublishedPostCandidates(db),
    selectDraftPostCandidates(db),
  )
    .orderBy(asc(sql`slug`))
    .limit(limit)
    .all();
}

export function listAuthorsWhoCanPublish(
  db: DxDrizzleDatabase,
): AuthorPublishPermissionRow[] {
  return intersect(
    selectAuthorsWithPublishedPosts(db),
    selectAdminAuthors(db),
  )
    .orderBy(asc(sql`email`))
    .all();
}

export function listUnpublishedPostIdentities(
  db: DxDrizzleDatabase,
  limit = 25,
): PostIdentityRow[] {
  return except(
    selectAllPostIdentities(db),
    selectPublishedPostIdentities(db),
  )
    .orderBy(asc(sql`slug`))
    .limit(limit)
    .all();
}
"#;

const DRIZZLE_CTE_QUERIES_TS: &str = r#"import { desc, eq, sql } from "drizzle-orm";

import type { DxDrizzleDatabase } from "./client";
import { posts, users } from "./schema";

export type AuthorPostCountRow = {
  id: number;
  email: string;
  name: string;
  postCount: number;
};

export type RecentPublishedPostSlug = {
  id: number;
  slug: string;
  title: string;
  publishedAt: Date | null;
};

export function listAuthorsWithPostCounts(
  db: DxDrizzleDatabase,
  limit = 25,
): AuthorPostCountRow[] {
  const postCounts = db.$with("post_counts").as(
    db
      .select({
        authorId: posts.authorId,
        postCount: sql<number>`count(${posts.id})`.as("post_count"),
      })
      .from(posts)
      .groupBy(posts.authorId),
  );

  return db
    .with(postCounts)
    .select({
      id: users.id,
      email: users.email,
      name: users.name,
      postCount: sql<number>`coalesce(${postCounts.postCount}, 0)`,
    })
    .from(users)
    .leftJoin(postCounts, eq(users.id, postCounts.authorId))
    .orderBy(desc(sql`coalesce(${postCounts.postCount}, 0)`), desc(users.createdAt))
    .limit(limit)
    .all();
}

export function listRecentPublishedPostSlugs(
  db: DxDrizzleDatabase,
  limit = 10,
): RecentPublishedPostSlug[] {
  const recentPublishedPosts = db
    .select({
      id: posts.id,
      slug: posts.slug,
      title: posts.title,
      publishedAt: posts.publishedAt,
    })
    .from(posts)
    .where(eq(posts.status, "published"))
    .orderBy(desc(posts.publishedAt))
    .limit(limit)
    .as("recent_published_posts");

  return db
    .select({
      id: recentPublishedPosts.id,
      slug: recentPublishedPosts.slug,
      title: recentPublishedPosts.title,
      publishedAt: recentPublishedPosts.publishedAt,
    })
    .from(recentPublishedPosts)
    .all();
}
"#;

const DRIZZLE_TRANSACTIONS_TS: &str = r#"import type { SQLiteTransactionConfig } from "drizzle-orm/sqlite-core";

import type { DxDrizzleDatabase } from "./client";
import { posts, users, type NewPost, type NewUser } from "./schema";

export type CreateUserWithPostInput = {
  user: NewUser;
  post: Omit<NewPost, "authorId">;
  transaction?: SQLiteTransactionConfig;
};

export function createUserWithPost(
  db: DxDrizzleDatabase,
  input: CreateUserWithPostInput,
) {
  return db.transaction((tx) => {
    const user = tx.insert(users).values(input.user).returning().get();
    const post = tx.insert(posts).values({
      ...input.post,
      authorId: user.id,
    }).returning().get();

    return { user, post };
  }, input.transaction);
}
"#;

const DRIZZLE_PREPARED_QUERIES_TS: &str = r#"import { desc, eq, placeholder } from "drizzle-orm";

import type { DxDrizzleDatabase } from "./client";
import { posts, users } from "./schema";

export type PreparedUsersByRoleInput = {
  role: "admin" | "member";
  limit: number;
};

export type PreparedPostBySlugInput = {
  slug: string;
};

export function prepareUsersByRole(db: DxDrizzleDatabase) {
  return db
    .select({
      id: users.id,
      email: users.email,
      name: users.name,
      role: users.role,
      createdAt: users.createdAt,
    })
    .from(users)
    .where(eq(users.role, placeholder("role")))
    .orderBy(desc(users.createdAt))
    .limit(placeholder("limit"))
    .prepare();
}

export function listPreparedUsersByRole(
  db: DxDrizzleDatabase,
  input: PreparedUsersByRoleInput,
) {
  return prepareUsersByRole(db).all(input);
}

export function preparePostBySlug(db: DxDrizzleDatabase) {
  return db
    .select({
      id: posts.id,
      slug: posts.slug,
      title: posts.title,
      status: posts.status,
      authorId: posts.authorId,
      publishedAt: posts.publishedAt,
    })
    .from(posts)
    .where(eq(posts.slug, placeholder("slug")))
    .limit(1)
    .prepare();
}

export function getPreparedPostBySlug(
  db: DxDrizzleDatabase,
  input: PreparedPostBySlugInput,
) {
  return preparePostBySlug(db).get(input);
}
"#;

const DRIZZLE_UPSERTS_TS: &str = r#"import { sql } from "drizzle-orm";

import type { DxDrizzleDatabase } from "./client";
import { posts, users, type NewPost, type NewUser } from "./schema";

export type UpsertUserByEmailInput = NewUser;
export type CreateUserIfAbsentInput = NewUser;
export type UpsertPostBySlugInput = NewPost;

export function upsertUserByEmail(
  db: DxDrizzleDatabase,
  input: UpsertUserByEmailInput,
) {
  return db
    .insert(users)
    .values(input)
    .onConflictDoUpdate({
      target: users.email,
      set: {
        name: sql`excluded.name`,
        role: sql`excluded.role`,
        updatedAt: sql`excluded.updated_at`,
      },
    })
    .returning()
    .get();
}

export function createUserIfAbsent(
  db: DxDrizzleDatabase,
  input: CreateUserIfAbsentInput,
) {
  return db
    .insert(users)
    .values(input)
    .onConflictDoNothing({ target: users.email })
    .returning()
    .get();
}

export function upsertPostBySlug(
  db: DxDrizzleDatabase,
  input: UpsertPostBySlugInput,
) {
  return db
    .insert(posts)
    .values(input)
    .onConflictDoUpdate({
      target: posts.slug,
      set: {
        authorId: sql`excluded.author_id`,
        title: sql`excluded.title`,
        body: sql`excluded.body`,
        status: sql`excluded.status`,
        publishedAt: sql`excluded.published_at`,
        updatedAt: sql`excluded.updated_at`,
      },
    })
    .returning()
    .get();
}
"#;

const DRIZZLE_MUTATIONS_TS: &str = r#"import { eq } from "drizzle-orm";

import type { DxDrizzleDatabase } from "./client";
import { posts, users } from "./schema";

export type UpdateUserRoleInput = {
  email: string;
  role: "admin" | "member";
};

export type PublishPostInput = {
  slug: string;
  publishedAt?: Date;
};

export function updateUserRole(
  db: DxDrizzleDatabase,
  input: UpdateUserRoleInput,
) {
  return db
    .update(users)
    .set({
      role: input.role,
      updatedAt: new Date(),
    })
    .where(eq(users.email, input.email))
    .returning()
    .get();
}

export function publishPost(
  db: DxDrizzleDatabase,
  input: PublishPostInput,
) {
  return db
    .update(posts)
    .set({
      status: "published",
      publishedAt: input.publishedAt ?? new Date(),
      updatedAt: new Date(),
    })
    .where(eq(posts.slug, input.slug))
    .returning()
    .get();
}

export function deletePostBySlug(db: DxDrizzleDatabase, slug: string) {
  return db
    .delete(posts)
    .where(eq(posts.slug, slug))
    .returning()
    .get();
}
"#;

const DRIZZLE_ANALYTICS_TS: &str = r#"import { avg, count, countDistinct, eq, sql } from "drizzle-orm";

import type { DxDrizzleDatabase } from "./client";
import { posts, users } from "./schema";

export type LaunchDatabaseStats = {
  users: number;
  distinctRoles: number;
  posts: number;
  publishedPosts: number;
  averagePostId: string | null;
};

function readCount(row: { value: number } | undefined): number {
  return row?.value ?? 0;
}

export function readLaunchDatabaseStats(db: DxDrizzleDatabase): LaunchDatabaseStats {
  const usersCount = db.select({ value: count() }).from(users).get();
  const rolesCount = db.select({ value: countDistinct(users.role) }).from(users).get();
  const postsCount = db.select({ value: count() }).from(posts).get();
  const publishedCount = db
    .select({
      value: sql<number>`count(*) filter (where ${posts.status} = 'published')`,
    })
    .from(posts)
    .get();
  const averagePostId = db.select({ value: avg(posts.id) }).from(posts).get();

  return {
    users: readCount(usersCount),
    distinctRoles: readCount(rolesCount),
    posts: readCount(postsCount),
    publishedPosts: readCount(publishedCount),
    averagePostId: averagePostId?.value ?? null,
  };
}

export function countPostsByStatus(
  db: DxDrizzleDatabase,
  status: "draft" | "published" | "archived",
) {
  return db
    .select({ value: count() })
    .from(posts)
    .where(eq(posts.status, status))
    .get()?.value ?? 0;
}
"#;

const DRIZZLE_REPLICAS_TS: &str = r#"import { withReplicas } from "drizzle-orm/sqlite-core";

import type { DxDrizzleDatabase } from "./client";

export type DxDrizzleReplicaRoutingStatus = "missing-replica" | "configured";

export type DxDrizzleReplicaReadiness = {
  packageId: "db/drizzle-sqlite";
  officialPackageName: "Database ORM";
  upstreamPackage: "drizzle-orm";
  status: DxDrizzleReplicaRoutingStatus;
  replicaCount: number;
  publicApi: "withReplicas";
  readApis: readonly ["select", "selectDistinct", "$count", "with", "query"];
  writeApis: readonly ["insert", "update", "delete", "transaction", "run"];
  appOwned: readonly string[];
};

export type DxDrizzleReplicaChooser = (
  replicas: DxDrizzleDatabase[],
) => DxDrizzleDatabase;

export type DxDrizzleReplicaSet = ReturnType<typeof createDxDrizzleReplicaSet>;

export function createDxDrizzleReplicaSet(
  primary: DxDrizzleDatabase,
  replicas: [DxDrizzleDatabase, ...DxDrizzleDatabase[]],
  chooseReplica: DxDrizzleReplicaChooser = (availableReplicas) => availableReplicas[0]!,
) {
  return withReplicas(primary, replicas, chooseReplica);
}

export function readDxDrizzleReplicaReadiness(
  replicaCount = 0,
): DxDrizzleReplicaReadiness {
  return {
    packageId: "db/drizzle-sqlite",
    officialPackageName: "Database ORM",
    upstreamPackage: "drizzle-orm",
    status: replicaCount > 0 ? "configured" : "missing-replica",
    replicaCount,
    publicApi: "withReplicas",
    readApis: ["select", "selectDistinct", "$count", "with", "query"],
    writeApis: ["insert", "update", "delete", "transaction", "run"],
    appOwned: [
      "Read-replica topology, replica health, routing policy, and write-after-read consistency stay app-owned",
      "SQLite file replication, snapshot freshness, and failover behavior stay app-owned",
      "Replica chooser instrumentation and production observability stay app-owned",
    ],
  } as const;
}
"#;

const DRIZZLE_DASHBOARD_WORKFLOW_TS: &str = r#"import { count, countDistinct, desc, eq, sql } from "drizzle-orm";

import type { DxDrizzleDatabase } from "./client";
import { readLaunchDatabaseStats, type LaunchDatabaseStats } from "./analytics";
import { listAuthorsWithPostCounts, type AuthorPostCountRow } from "./cte-queries";
import { listPublishedPostPreviews, type PublishedPostPreview } from "./joins";
import { listUsers } from "./queries";
import { posts, users, type User } from "./schema";

export type DrizzleDashboardOverview = {
  stats: LaunchDatabaseStats;
  previews: PublishedPostPreview[];
  authorCounts: AuthorPostCountRow[];
  recentUsers: User[];
};

export type DrizzleDashboardOverviewOptions = {
  previewLimit?: number;
  userLimit?: number;
};

export type DrizzleDashboardQueryPlanId = "overview" | "published-posts" | "author-counts";

export type DrizzleDashboardQueryPlan = {
  id: DrizzleDashboardQueryPlanId;
  helper: string;
  publicApi: string;
  sql: string;
  params: unknown[];
};

type DrizzleQueryPreview = {
  sql: string;
  params?: unknown[];
};

export function readDrizzleDashboardOverview(
  db: DxDrizzleDatabase,
  options: DrizzleDashboardOverviewOptions = {},
): DrizzleDashboardOverview {
  return {
    stats: readLaunchDatabaseStats(db),
    previews: listPublishedPostPreviews(db, options.previewLimit ?? 6),
    authorCounts: listAuthorsWithPostCounts(db),
    recentUsers: listUsers(db, { limit: options.userLimit ?? 6 }),
  };
}

function normalizeDrizzleQueryPreview(
  query: DrizzleQueryPreview,
): Pick<DrizzleDashboardQueryPlan, "sql" | "params"> {
  return {
    sql: query.sql,
    params: query.params ?? [],
  };
}

export function readDrizzleDashboardQueryPlan(
  db: DxDrizzleDatabase,
): DrizzleDashboardQueryPlan[] {
  const overview = db
    .select({
      users: countDistinct(users.id),
      distinctRoles: countDistinct(users.role),
      posts: count(posts.id),
      publishedPosts: sql<number>`count(*) filter (where ${posts.status} = 'published')`,
    })
    .from(users)
    .leftJoin(posts, eq(users.id, posts.authorId))
    .toSQL();

  const publishedPosts = db
    .select({
      id: posts.id,
      slug: posts.slug,
      title: posts.title,
      authorName: users.name,
      publishedAt: posts.publishedAt,
      bodyLength: sql<number>`length(${posts.body})`,
    })
    .from(posts)
    .innerJoin(users, eq(posts.authorId, users.id))
    .where(eq(posts.status, "published"))
    .orderBy(desc(posts.publishedAt))
    .limit(6)
    .toSQL();

  const postCount = sql<number>`count(${posts.id})`;
  const authorCounts = db
    .select({
      authorId: users.id,
      name: users.name,
      postCount,
    })
    .from(users)
    .leftJoin(posts, eq(users.id, posts.authorId))
    .groupBy(users.id)
    .orderBy(desc(postCount))
    .limit(6)
    .toSQL();

  return [
    {
      id: "overview",
      helper: "readLaunchDatabaseStats",
      publicApi: "count/countDistinct/sql aggregate + leftJoin + toSQL()",
      ...normalizeDrizzleQueryPreview(overview),
    },
    {
      id: "published-posts",
      helper: "listPublishedPostPreviews",
      publicApi: "select().from().innerJoin().toSQL()",
      ...normalizeDrizzleQueryPreview(publishedPosts),
    },
    {
      id: "author-counts",
      helper: "listAuthorsWithPostCounts",
      publicApi: "select().from().leftJoin().groupBy().toSQL()",
      ...normalizeDrizzleQueryPreview(authorCounts),
    },
  ];
}

export function getDrizzleDashboardQueryPlan(
  plans: readonly DrizzleDashboardQueryPlan[],
  id: DrizzleDashboardQueryPlanId,
): DrizzleDashboardQueryPlan {
  const plan = plans.find((plan) => plan.id === id);

  if (!plan) {
    throw new Error(`Unknown Drizzle dashboard query plan: ${id}`);
  }

  return plan;
}

export function readDrizzleDashboardQueryPlanById(
  db: DxDrizzleDatabase,
  id: DrizzleDashboardQueryPlanId,
): DrizzleDashboardQueryPlan {
  return getDrizzleDashboardQueryPlan(readDrizzleDashboardQueryPlan(db), id);
}

export function readDrizzleDashboardReadiness() {
  return {
    packageId: "db/drizzle-sqlite",
    entryPoint: "readDrizzleDashboardOverview",
    queryPlanEntryPoint: "readDrizzleDashboardQueryPlan",
    queryPlanByIdEntryPoint: "readDrizzleDashboardQueryPlanById",
    runtimeDependencies: ["drizzle-orm", "better-sqlite3"],
    appOwned: [
      "SQLite database path or DATABASE_URL",
      "reviewed Drizzle migration SQL",
      "dashboard authorization policy",
      "backup and retention policy",
    ],
  } as const;
}
"#;

const DRIZZLE_METADATA_TS: &str = r#"export const dxDrizzlePackage = {
  packageId: "db/drizzle-sqlite",
  officialPackageName: "Database ORM",
  aliases: ["database/drizzle", "db/drizzle", "drizzle", "drizzle-orm/sqlite", "drizzle/sqlite"],
  upstreamPackage: "drizzle-orm",
  upstreamVersion: "0.45.3",
  forgeVersion: "0.1.0",
  sourceMirror: "G:\\WWW\\inspirations\\drizzle-orm",
  provenance: {
    upstreamReference: "npm:drizzle-orm@0.45.3",
    repository: "https://github.com/drizzle-team/drizzle-orm",
    license: "Apache-2.0",
    inspectedSources: [
      "package.json",
      "drizzle-orm/src/better-sqlite3/driver.ts",
      "drizzle-orm/src/better-sqlite3/migrator.ts",
      "drizzle-orm/src/sqlite-core/table.ts",
      "drizzle-orm/src/sqlite-core/view.ts",
      "drizzle-orm/src/sqlite-core/query-builders/select.ts",
      "drizzle-orm/src/sqlite-core/query-builders/insert.ts",
      "drizzle-orm/src/sqlite-core/query-builders/update.ts",
      "drizzle-orm/src/sqlite-core/query-builders/delete.ts",
      "drizzle-orm/src/sqlite-core/db.ts",
      "drizzle-orm/src/relations.ts",
    ],
  },
  driverPackage: "better-sqlite3",
  driverImport: "drizzle-orm/better-sqlite3",
  runtimeDependencies: ["drizzle-orm", "better-sqlite3"],
  devDependencies: ["drizzle-kit", "@types/better-sqlite3"],
  requiredEnv: [],
  exportedFiles: [
    "db/drizzle/client.ts",
    "db/drizzle/schema.ts",
    "db/drizzle/views.ts",
    "db/drizzle/queries.ts",
    "db/drizzle/migrations.ts",
    "db/drizzle/relational-queries.ts",
    "db/drizzle/joins.ts",
    "db/drizzle/set-operations.ts",
    "db/drizzle/cte-queries.ts",
    "db/drizzle/transactions.ts",
    "db/drizzle/prepared-queries.ts",
    "db/drizzle/upserts.ts",
    "db/drizzle/mutations.ts",
    "db/drizzle/analytics.ts",
    "db/drizzle/replicas.ts",
    "db/drizzle/dashboard-workflow.ts",
    "db/drizzle/metadata.ts",
    "db/drizzle/README.md",
  ],
  receiptPaths: [
    ".dx/forge/docs/db-drizzle-sqlite.md",
    ".dx/forge/receipts/*-db-drizzle-sqlite.json",
    "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
    ".dx/forge/source-.dx/build-cache/manifest.json",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
    monitoredSurfaces: [
      {
        id: "drizzle-replica-routing",
        status: "present",
        sourceFile: "core/src/ecosystem/forge_drizzle.rs",
        materializedFile: "db/drizzle/replicas.ts",
        nextAction: "Keep upstream withReplicas, selectDistinct, and $count coverage in the Database ORM source guard.",
      },
      {
        id: "drizzle-launch-dashboard-workflow",
        status: "present",
        sourceFile: "examples/template/drizzle-query-proof.tsx",
        materializedFile: "components/launch/drizzle-query-proof.tsx",
        nextAction: "Keep Zed/DX Studio data-dx markers aligned with the visible Database ORM workflow.",
      },
    ],
  },
  dxStyleCompatibility: {
    schema: "dx.forge.package.dx_style_compatibility",
    status: "present",
    tokenSource: "tools/launch/runtime-template/assets/launch-runtime.css",
    generatedCss: "tools/launch/runtime-template/assets/launch-runtime.css",
    visibleSurfaces: ["launch-drizzle-data-workflow"],
    sourceFiles: [
      "examples/template/drizzle-query-proof.tsx",
      "tools/launch/runtime-template/assets/launch-runtime.css",
    ],
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
    runtimeProof: false,
    runtimeLimitations: [
      "source-only dx-style token compatibility; no browser visual proof is claimed",
      "live SQLite data rendering remains app-owned",
      "theme token review remains app-owned",
    ],
  },
  honestyLabel: "SOURCE-ONLY",
  dxIcon: "pack:database",
  primaryExports: [
    "createDxDrizzleConnection",
    "users",
    "posts",
    "usersRelations",
    "postsRelations",
    "publishedPostSummaries",
    "existingPublishedPostSummaries",
    "listPublishedPostSummaries",
    "readPublishedPostSummaryFields",
    "createUser",
    "findUserByEmail",
    "listUsers",
    "createPost",
    "listPublishedPosts",
    "buildDxDrizzleMigrationConfig",
    "applyDxDrizzleMigrations",
    "listUsersWithPosts",
    "findPostWithAuthor",
    "listPublishedPostPreviews",
    "listUsersWithOptionalPosts",
    "listLaunchAudience",
    "listPublicationCandidates",
    "listAuthorsWhoCanPublish",
    "listUnpublishedPostIdentities",
    "listAuthorsWithPostCounts",
    "listRecentPublishedPostSlugs",
    "createUserWithPost",
    "prepareUsersByRole",
    "listPreparedUsersByRole",
    "preparePostBySlug",
    "getPreparedPostBySlug",
    "upsertUserByEmail",
    "createUserIfAbsent",
    "upsertPostBySlug",
    "updateUserRole",
    "publishPost",
    "deletePostBySlug",
    "readLaunchDatabaseStats",
    "countPostsByStatus",
    "createDxDrizzleReplicaSet",
    "readDxDrizzleReplicaReadiness",
    "readDrizzleDashboardOverview",
    "readDrizzleDashboardQueryPlan",
    "getDrizzleDashboardQueryPlan",
    "readDrizzleDashboardQueryPlanById",
    "readDrizzleDashboardReadiness",
  ],
  publicApis: [
    "sqliteTable",
    "relations",
    "InferSelectModel",
    "InferInsertModel",
    "eq",
    "and",
    "desc",
    "sql",
    "drizzle",
    "migrate",
    "MigrationConfig",
    "sqliteView",
    "sqliteView.existing",
    "getViewSelectedFields",
    "db.query.*.findMany/findFirst",
    "leftJoin",
    "innerJoin",
    "union",
    "unionAll",
    "intersect",
    "except",
    "db.$with",
    "db.with",
    "subquery.as",
    "groupBy",
    "SQLiteTransactionConfig",
    "db.transaction",
    "placeholder",
    "placeholder() + .prepare()",
    "onConflictDoUpdate",
    "onConflictDoNothing",
    "returning",
    "db.update().set().where().returning()",
    "db.delete().where().returning()",
    "count",
    "countDistinct",
    "avg",
    "sql aggregate",
    "toSQL",
    "withReplicas",
    "selectDistinct",
    "$count",
  ],
  migrations: {
    helper: "applyDxDrizzleMigrations",
    configBuilder: "buildDxDrizzleMigrationConfig",
    publicApi: "drizzle-orm/better-sqlite3/migrator",
    defaultFolder: "./drizzle",
  },
  views: {
    publishedSummaries: "publishedPostSummaries",
    existingSummaries: "existingPublishedPostSummaries",
    listPublishedPostSummaries: "listPublishedPostSummaries",
    readSelectedFields: "readPublishedPostSummaryFields",
    publicApi: "sqliteView/sqliteView.existing/getViewSelectedFields",
  },
  relationalQueries: {
    listUsers: "listUsersWithPosts",
    findPost: "findPostWithAuthor",
    publicApi: "db.query.*.findMany/findFirst",
    relations: ["users.posts", "posts.author"],
  },
  joins: {
    listPostPreviews: "listPublishedPostPreviews",
    listUsersWithOptionalPosts: "listUsersWithOptionalPosts",
    publicApi: "select().from().leftJoin/innerJoin",
    nullableSide: "leftJoin posts",
  },
  setOperations: {
    listAudience: "listLaunchAudience",
    listPublicationCandidates: "listPublicationCandidates",
    listAuthorsWhoCanPublish: "listAuthorsWhoCanPublish",
    listUnpublishedPostIdentities: "listUnpublishedPostIdentities",
    publicApi: "union/unionAll/intersect/except",
    duplicatePolicy: "union removes duplicates, unionAll keeps duplicates",
  },
  cteQueries: {
    listAuthorsWithPostCounts: "listAuthorsWithPostCounts",
    listRecentPublishedPostSlugs: "listRecentPublishedPostSlugs",
    publicApi: "db.$with/db.with/subquery.as",
    aliases: ["post_counts", "recent_published_posts"],
  },
  transactions: {
    transactionHelper: "createUserWithPost",
    publicApi: "db.transaction",
    configType: "SQLiteTransactionConfig",
    atomicWrites: ["users", "posts"],
  },
  preparedQueries: {
    listUsers: "listPreparedUsersByRole",
    getPost: "getPreparedPostBySlug",
    publicApi: "placeholder() + .prepare()",
    statements: ["prepareUsersByRole", "preparePostBySlug"],
  },
  conflictWrites: {
    upsertUser: "upsertUserByEmail",
    createUserIfAbsent: "createUserIfAbsent",
    upsertPost: "upsertPostBySlug",
    publicApi: "onConflictDoUpdate/onConflictDoNothing",
    conflictTargets: ["users.email", "posts.slug"],
  },
  mutations: {
    updateUser: "updateUserRole",
    publishPost: "publishPost",
    deletePost: "deletePostBySlug",
    publicApi: "db.update/db.delete returning",
  },
  analytics: {
    readStats: "readLaunchDatabaseStats",
    countPostsByStatus: "countPostsByStatus",
    publicApi: "count/countDistinct/avg/sql aggregate",
    metrics: ["users", "distinctRoles", "posts", "publishedPosts", "averagePostId"],
  },
  replicaRouting: {
    createReplicaSet: "createDxDrizzleReplicaSet",
    readReadiness: "readDxDrizzleReplicaReadiness",
    publicApi: "withReplicas",
    readApis: ["select", "selectDistinct", "$count", "with", "query"],
    writeApis: ["insert", "update", "delete", "transaction", "run"],
  },
  dashboardUsage: {
    sourceFile: "examples/dashboard/src/components/DrizzleDashboardWorkflow.tsx",
    visibleComponent: "DrizzleDashboardWorkflow",
    workflow: "content-readiness",
    packageMarker: 'data-dx-package="db/drizzle-sqlite"',
    componentMarker: 'data-dx-component="dashboard-drizzle-workflow"',
    interactions: ["select-dashboard-query", "preview-dashboard-query-plan", "prepare-dashboard-query"],
    entryPoint: "readDrizzleDashboardOverview",
    queryPlanEntryPoint: "readDrizzleDashboardQueryPlan",
    queryPlanByIdEntryPoint: "readDrizzleDashboardQueryPlanById",
    receiptPathMarker: 'data-dx-drizzle-receipt-path',
    runtimeDependenciesMarker: 'data-dx-drizzle-runtime-dependencies',
  },
  launchUsage: {
    sourceFile: "examples/template/drizzle-query-proof.tsx",
    visibleComponent: "LaunchDrizzleDashboardData",
    workflow: "sqlite-read-model",
    packageMarker: 'data-dx-package="db/drizzle-sqlite"',
    componentMarker: 'data-dx-component="launch-drizzle-data-workflow"',
    sourceMarker: 'data-dx-source="examples/template/drizzle-query-proof.tsx"',
    interactions: ["select-read-model", "preview-query-plan", "apply-read-model"],
    productSurface: "launch-data-dashboard",
    missionControlTarget: "mission-control-database",
    backendStatusMarker: 'data-dx-backend-status',
    backendDetailMarker: 'data-dx-backend-detail',
    receiptPathMarker: 'data-dx-drizzle-receipt-path',
    receiptState: 'data-dx-drizzle-receipt-state',
    runtimeDependenciesMarker: 'data-dx-drizzle-runtime-dependencies',
  },
  appOwnedBoundaries: [
    "Migration SQL files and Drizzle Kit generation stay app-owned",
    "View SQL definitions, migration lifecycle, and compatibility with existing database views stay app-owned",
    "Relational query shape stays schema-owned and should change with app tables",
    "Join shape, null-handling, and cross-table authorization stay app-owned",
    "Set operation operand order, duplicate policy, and pagination stay app-owned",
    "CTE names, SQL aliases, aggregation semantics, and subquery pagination stay app-owned",
    "Atomic write helpers stay app-owned when business rules diverge",
    "Prepared statement lifetime and invalidation stay app-owned",
    "Conflict targets and merge policy stay app-owned",
    "Mutation authorization, audit trail, and soft-delete policy stay app-owned",
    "Analytics definitions and business KPIs stay app-owned",
    "Read-replica topology, replica health, routing policy, and write-after-read consistency stay app-owned",
    "Database path, backups, permissions, and deployed data policy stay app-owned",
    "Driver changes beyond better-sqlite3 stay app-owned",
  ],
} as const;

export type DxDrizzlePackageMetadata = typeof dxDrizzlePackage;
"#;

const DRIZZLE_README_MD: &str = r#"# Database ORM

This source-owned slice is based on the real `drizzle-orm` public API observed in the local inspiration mirror at version `drizzle-orm 0.45.3`.

It uses:

- `drizzle-orm/sqlite-core` for `sqliteTable`, `text`, `integer`, `index`, and `uniqueIndex`
- `drizzle-orm` for `relations`, `InferSelectModel`, `InferInsertModel`, `eq`, `and`, `desc`, and `sql`
- `drizzle-orm/better-sqlite3` for the launch-friendly local SQLite client
- `drizzle-orm/better-sqlite3/migrator` for the real Better SQLite migration runner
- SQLite views through `sqliteView(...)`, `sqliteView(...).existing()`, typed `$inferSelect`, and `getViewSelectedFields(...)`
- Drizzle relational query builder through `db.query.users.findMany({ with: { posts: true } })`
- SQLite joins through `select().from(...).innerJoin(...)` and nullable `leftJoin(...)`
- SQLite set operations through `union(...)`, `unionAll(...)`, `intersect(...)`, and `except(...)`
- SQLite CTEs and subqueries through `db.$with(...)`, `db.with(...)`, `sql(...).as(...)`, and `.as("subquery")`
- SQLite transactions through `db.transaction((tx) => ...)` and `SQLiteTransactionConfig`
- Prepared SQLite statements through `placeholder(...)`, `.prepare()`, `.all(...)`, and `.get(...)`
- SQLite conflict writes through `onConflictDoUpdate(...)`, `onConflictDoNothing(...)`, and `returning()`
- SQLite mutations through `db.update(...).set(...).where(...).returning()` and `db.delete(...).where(...).returning()`
- SQLite analytics through `count()`, `countDistinct(...)`, `avg(...)`, and typed `sql` aggregate expressions
- SQLite replica routing through `withReplicas(...)`, routing reads through `select`, `selectDistinct`, `$count`, `with`, and `query` while keeping writes on the primary
- A dashboard workflow helper through `readDrizzleDashboardOverview(...)` that composes stats, post previews, author counts, and recent users for starter dashboards
- A dashboard query-plan helper through `readDrizzleDashboardQueryPlan(...)` that uses real Drizzle builders and `.toSQL()` for safe SQL/params preview before app-owned execution
- A dashboard query-plan selector through `readDrizzleDashboardQueryPlanById(...)` and `getDrizzleDashboardQueryPlan(...)` so dashboards can preview one typed read surface without open-coding plan lookup
- A launch data workflow through `LaunchDrizzleDashboardData` that lets the generated `/launch` dashboard select a SQLite read model, preview the query plan, and prepare a local receipt
- `metadata.ts` for stable DX CLI, Zed, and template discovery

Runtime dependency policy:

Forge does not run package-manager installs or create `node_modules` while
materializing this template. If the host app chooses to execute this Drizzle
slice, review and add these app-owned dependencies through that app's governed
package process:

- runtime: `drizzle-orm`, `better-sqlite3`
- migration/development: `drizzle-kit`, `@types/better-sqlite3`

Example:

```ts
import { readLaunchDatabaseStats } from "@/db/drizzle/analytics";
import { createDxDrizzleConnection } from "@/db/drizzle/client";
import { listAuthorsWithPostCounts } from "@/db/drizzle/cte-queries";
import {
  readDrizzleDashboardOverview,
  readDrizzleDashboardQueryPlan,
  readDrizzleDashboardQueryPlanById,
} from "@/db/drizzle/dashboard-workflow";
import { listPublishedPostPreviews } from "@/db/drizzle/joins";
import { applyDxDrizzleMigrations } from "@/db/drizzle/migrations";
import { updateUserRole } from "@/db/drizzle/mutations";
import { listPreparedUsersByRole } from "@/db/drizzle/prepared-queries";
import { readDxDrizzleReplicaReadiness } from "@/db/drizzle/replicas";
import { listUsers } from "@/db/drizzle/queries";
import { listUsersWithPosts } from "@/db/drizzle/relational-queries";
import { listLaunchAudience } from "@/db/drizzle/set-operations";
import { createUserWithPost } from "@/db/drizzle/transactions";
import { upsertUserByEmail } from "@/db/drizzle/upserts";
import { listPublishedPostSummaries } from "@/db/drizzle/views";

const { db, sqlite } = createDxDrizzleConnection(process.env.DATABASE_URL ?? "sqlite.db");

applyDxDrizzleMigrations(db, { migrationsFolder: "./drizzle" });

const { user } = createUserWithPost(db, {
  user: {
    email: "launch@example.com",
    name: "Launch Operator",
  },
  post: {
    slug: "launch-note",
    title: "Launch note",
    body: "DX owns the source slice; the app owns rollout policy.",
    status: "draft",
  },
});

const preparedMembers = listPreparedUsersByRole(db, { role: "member", limit: 10 });
const upsertedUser = upsertUserByEmail(db, {
  email: "launch@example.com",
  name: "Launch Operator",
  role: "member",
});
const promotedUser = updateUserRole(db, {
  email: "launch@example.com",
  role: "admin",
});
const stats = readLaunchDatabaseStats(db);
const previews = listPublishedPostPreviews(db);
const audience = listLaunchAudience(db);
const authorCounts = listAuthorsWithPostCounts(db);
const viewSummaries = listPublishedPostSummaries(db);
const dashboardOverview = readDrizzleDashboardOverview(db);
const queryPlan = readDrizzleDashboardQueryPlan(db);
const selectedQueryPlan = readDrizzleDashboardQueryPlanById(db, "overview");
const replicaReadiness = readDxDrizzleReplicaReadiness(0);

console.log(user, listUsers(db, { role: "member" }), listUsersWithPosts(db), preparedMembers, upsertedUser, promotedUser, stats, previews, audience, authorCounts, viewSummaries, dashboardOverview, queryPlan, selectedQueryPlan, replicaReadiness);
sqlite.close();
```

What Forge owns:

- editable SQLite schema, view helpers, query helpers, relational query helpers, typed join helpers, set operation helpers, CTE/subquery helpers, transaction helper, prepared query helpers, conflict write helpers, mutation helpers, analytics helpers, replica-routing helper, dashboard workflow helper, migrator helper, and package metadata
- public Drizzle imports only; no vendored Drizzle internals
- launch-safe defaults for local `better-sqlite3`

What the app owns:

- generating and reviewing migration SQL with Drizzle Kit
- the `./drizzle` migration folder contents and rollout order
- view SQL definitions, migration lifecycle, and compatibility with existing database views
- evolving relational query shapes with application tables and access policy
- join shape, null-handling, selected columns, and cross-table authorization
- set operation operand order, duplicate policy, result ordering, and pagination
- CTE names, SQL aliases, aggregation semantics, and subquery pagination
- evolving transaction boundaries, retries, and business-rule atomicity with application workflows
- prepared statement lifetime, invalidation, and hot-reload/runtime caching policy
- conflict targets, merge policy, audit behavior, and idempotency rules
- mutation authorization, audit trail, and soft-delete policy
- analytics definitions, business KPIs, aggregation windows, and reporting policy
- read-replica topology, replica health, routing policy, and write-after-read consistency
- query-plan display policy, explain-plan review, index decisions, and SQL logging/redaction policy
- database file location, backups, permissions, and deployed data policy

Intentionally deferred:

- PostgreSQL, MySQL, LibSQL, D1, and hosted-driver adapters
- automatic migration execution during template generation
- production view lifecycle orchestration and generated SQL review
- production transaction retries and concurrency orchestration
- prepared statement pool lifecycle orchestration
- production set-operation dedupe, cursor, and feed-merge policy
- production CTE materialization, query-plan, and cursor policy
- production idempotency keys and write-audit orchestration
- domain-specific authorization and audit logging around mutations
- production analytics pipelines, rollups, and warehouse sync
- production read-replica provisioning, health checks, failover, and consistency policy
- production query-plan analysis, EXPLAIN output, and SQL telemetry
- production backup/rollback orchestration
"#;
