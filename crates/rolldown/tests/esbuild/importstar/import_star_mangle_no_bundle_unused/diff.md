# Diff
## /out.js
### esbuild
```js
import "./foo";
let foo = 234;
console.log(foo);
```
### rolldown
```js
import "./foo";

//#region entry.js
let foo = 234;
console.log(foo);

//#endregion

```
### diff
```diff
===================================================================
--- esbuild	/out.js
+++ rolldown	entry_js.js
@@ -1,3 +1,3 @@
 import "./foo";
-let foo = 234;
+var foo = 234;
 console.log(foo);

```