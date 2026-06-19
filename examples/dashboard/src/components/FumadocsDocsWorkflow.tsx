import { useState } from 'dx';
import {
    createFumadocsNavigationReceipt,
    fumadocsDashboardContract,
    fumadocsDashboardPages,
    getFumadocsDashboardPage,
    type FumadocsDashboardPageId,
    type FumadocsNavigationReceipt,
} from '../lib/fumadocsDocsWorkflow';

export function FumadocsDocsWorkflow() {
    const [pageId, setPageId] = useState<FumadocsDashboardPageId>('overview');
    const [receipt, setReceipt] = useState<FumadocsNavigationReceipt | null>(null);
    const activePage = getFumadocsDashboardPage(pageId);

    const selectPage = (nextPageId: FumadocsDashboardPageId) => {
        setPageId(nextPageId);
        setReceipt(null);
    };

    const previewRoute = () => {
        setReceipt(createFumadocsNavigationReceipt({ pageId }));
    };

    return (
        <section
            class="docs-workflow-panel"
            data-dx-package="content/fumadocs-next"
            data-dx-component="dashboard-fumadocs-docs-workflow"
            data-dx-fumadocs-dashboard-workflow="docs-ops"
            data-dx-style-surface="documentation-system"
            data-dx-fumadocs-route={activePage.route}
            data-dx-fumadocs-selected-page={activePage.id}
            data-dx-node-modules="forbidden"
        >
            <header class="panel-header">
                <dx-icon name="pack:fumadocs" aria-label="Documentation System" />
                <div>
                    <h2>Documentation System Workflow</h2>
                    <p>Operate docs, OpenAPI, search, and LLM handoff routes from the source-owned package slice.</p>
                </div>
            </header>

            <div class="provider-options" data-dx-fumadocs-interaction="page-tree-selector">
                {fumadocsDashboardPages.map((page) => {
                    const selected = page.id === activePage.id;

                    return (
                        <button
                            key={page.id}
                            type="button"
                            class={selected ? 'active' : ''}
                            data-dx-fumadocs-page-option={page.id}
                            data-dx-fumadocs-page-route={page.route}
                            data-dx-fumadocs-page-selected={selected ? 'true' : 'false'}
                            aria-pressed={selected ? 'true' : 'false'}
                            onClick={() => selectPage(page.id)}
                        >
                            <dx-icon name="pack:docs" aria-hidden="true" />
                            {page.title}
                        </button>
                    );
                })}
            </div>

            <dl class="readiness-list" data-dx-fumadocs-navigation-snapshot={activePage.breadcrumb.join('/')}>
                <div>
                    <dt>Route</dt>
                    <dd data-dx-fumadocs-route-contract={activePage.route}>{activePage.route}</dd>
                </div>
                <div>
                    <dt>Breadcrumb</dt>
                    <dd data-dx-fumadocs-breadcrumb={activePage.breadcrumb.join('/')}>
                        {activePage.breadcrumb.join(' / ')}
                    </dd>
                </div>
                <div>
                    <dt>Required env</dt>
                    <dd data-dx-fumadocs-required-env={fumadocsDashboardContract.requiredEnv.join(',')}>
                        {fumadocsDashboardContract.requiredEnv.join(', ')}
                    </dd>
                </div>
                <div>
                    <dt>Public APIs</dt>
                    <dd data-dx-fumadocs-public-api={fumadocsDashboardContract.upstreamPublicApis.join(',')}>
                        {fumadocsDashboardContract.upstreamPublicApis.slice(0, 3).join(', ')}
                    </dd>
                </div>
            </dl>

            <div class="docs-workflow-body" data-dx-fumadocs-rendered-markdown="active-dashboard-page">
                <h3>{activePage.title}</h3>
                <p>{activePage.description}</p>
                <ul data-dx-fumadocs-toc-list={activePage.id}>
                    {activePage.toc.map((item) => (
                        <li key={item} data-dx-fumadocs-toc-item={item}>
                            {item}
                        </li>
                    ))}
                </ul>
                <p data-dx-fumadocs-peer-count={String(activePage.peers.length)}>
                    Related pages: {activePage.peers.join(', ')}
                </p>
            </div>

            <button
                type="button"
                class="primary-action"
                data-dx-fumadocs-action="safe-local-route-preview"
                onClick={previewRoute}
            >
                <dx-icon name="pack:search" aria-hidden="true" />
                Preview route receipt
            </button>

            <p
                class="assistant-receipt"
                role="status"
                aria-live="polite"
                data-dx-fumadocs-local-response={receipt ? receipt.status : 'idle'}
                data-dx-fumadocs-receipt-route={receipt ? receipt.route : 'none'}
            >
                {receipt
                    ? `${receipt.packageId} selected ${receipt.route} with ${receipt.tocCount} TOC entries. ${receipt.nextAction}`
                    : 'Select a docs route, then preview the local Documentation System route receipt.'}
            </p>
        </section>
    );
}
