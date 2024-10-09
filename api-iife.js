if ("__TAURI__" in window) {
	var __TAURI_PLUGIN_HTTP__ = (function (e) {
		"use strict";
		async function t(e, t = {}, r) {
			return window.__TAURI_INTERNALS__.invoke(e, t, r);
		}
		"function" == typeof SuppressedError && SuppressedError;
		const r = "Request canceled";
		return (
			(e.fetch = async function (e, n) {
				const a = n?.signal;
				if (a?.aborted) throw new Error(r);
				const o = n?.maxRedirections,
					s = n?.connectTimeout,
					i = n?.proxy;
				n &&
					(delete n.maxRedirections,
					delete n.connectTimeout,
					delete n.proxy);
				const d = n?.headers
						? n.headers instanceof Headers
							? n.headers
							: new Headers(n.headers)
						: new Headers(),
					c = new Request(e, n),
					u = await c.arrayBuffer(),
					f =
						0 !== u.byteLength
							? Array.from(new Uint8Array(u))
							: null;
				for (const [e, t] of c.headers) d.get(e) || d.set(e, t);
				const _ = (
					d instanceof Headers
						? Array.from(d.entries())
						: Array.isArray(d)
							? d
							: Object.entries(d)
				).map(([e, t]) => [e, "string" == typeof t ? t : t.toString()]);
				if (a?.aborted) throw new Error(r);
				const h = await t("plugin:http|fetch", {
						clientConfig: {
							method: c.method,
							url: c.url,
							headers: _,
							data: f,
							maxRedirections: o,
							connectTimeout: s,
							proxy: i,
						},
					}),
					l = () => t("plugin:http|fetch_cancel", { rid: h });
				if (a?.aborted) throw (l(), new Error(r));
				a?.addEventListener("abort", () => {
					l();
				});
				const {
						status: p,
						statusText: w,
						url: y,
						headers: T,
						rid: A,
					} = await t("plugin:http|fetch_send", { rid: h }),
					g = await t("plugin:http|fetch_read_body", { rid: A }),
					R = new Response(
						g instanceof ArrayBuffer && 0 !== g.byteLength
							? g
							: g instanceof Array && g.length > 0
								? new Uint8Array(g)
								: null,
						{ status: p, statusText: w },
					);
				return (
					Object.defineProperty(R, "url", { value: y }),
					Object.defineProperty(R, "headers", {
						value: new Headers(T),
					}),
					R
				);
			}),
			e
		);
	})({});
	Object.defineProperty(window.__TAURI__, "http", {
		value: __TAURI_PLUGIN_HTTP__,
	});
}
