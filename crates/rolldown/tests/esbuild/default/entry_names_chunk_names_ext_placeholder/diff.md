# Reason
1. custom diff resolver
# Diff
## /out/main/js/entry1-4X3SO762.js
### esbuild
```js
import "../../common/js/chunk-XHGYOYUR.js";

// src/entries/entry1.js
console.log("entry1");
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out/main/js/entry1-4X3SO762.js
+++ rolldown	
@@ -1,2 +0,0 @@
-import "../../common/js/chunk-XHGYOYUR.js";
-console.log("entry1");

```
## /out/main/js/entry2-URQRHZS5.js
### esbuild
```js
import "../../common/js/chunk-XHGYOYUR.js";

// src/entries/entry2.js
console.log("entry2");
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out/main/js/entry2-URQRHZS5.js
+++ rolldown	
@@ -1,2 +0,0 @@
-import "../../common/js/chunk-XHGYOYUR.js";
-console.log("entry2");

```
## /out/common/js/chunk-XHGYOYUR.js
### esbuild
```js
// src/lib/shared.js
console.log("shared");
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out/common/js/chunk-XHGYOYUR.js
+++ rolldown	
@@ -1,1 +0,0 @@
-console.log("shared");

```
## /out/main/css/entry1-3JZGIUSL.css
### esbuild
```js
/* src/entries/entry1.css */
a:after {
  content: "entry1";
}
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out/main/css/entry1-3JZGIUSL.css
+++ rolldown	
@@ -1,4 +0,0 @@
-/* src/entries/entry1.css */
-a:after {
-  content: "entry1";
-}
\ No newline at end of file

```
## /out/main/css/entry2-NXZBPPIA.css
### esbuild
```js
/* src/entries/entry2.css */
a:after {
  content: "entry2";
}
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out/main/css/entry2-NXZBPPIA.css
+++ rolldown	
@@ -1,4 +0,0 @@
-/* src/entries/entry2.css */
-a:after {
-  content: "entry2";
-}
\ No newline at end of file

```