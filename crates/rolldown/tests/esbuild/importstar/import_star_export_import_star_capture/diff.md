## /out.js
### esbuild
```js
// foo.js
var foo_exports = {};
__export(foo_exports, {
  foo: () => foo
});
var foo = 123;

// entry.js
var foo2 = 234;
console.log(foo_exports, foo_exports.foo, foo2);
```
### rolldown
```js
import { default as assert } from "node:assert";


//#region foo.js
var foo_exports = {};
__export(foo_exports, { foo: () => foo$1 });
const foo$1 = 123;

//#endregion
//#region entry.js
let foo = 234;
assert.deepEqual(foo_exports, { foo: 123 });
assert.equal(foo, 234);

//#endregion

```
### diff
```diff
===================================================================
--- esbuild	/out.js
+++ rolldown	entry_js.mjs
@@ -1,5 +1,6 @@
 var foo_exports = {};
-__export(foo_exports, { foo: () => foo });
-var foo = 123;
-var foo2 = 234;
-console.log(foo_exports, foo_exports.foo, foo2);
\ No newline at end of file
+__export(foo_exports, { foo: () => foo$1 });
+const foo$1 = 123;
+let foo = 234;
+console.log(foo_exports);
+console.log(foo);
\ No newline at end of file

```