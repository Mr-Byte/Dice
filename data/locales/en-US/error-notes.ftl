note-prefix = note: 

index-operator-types = The index operator only accepts integers, strings, or symbols.
module-load-error = This issue occurred while trying to load the module "{$module}".
export-only-allowed-in-modules = The 'export' keyword can only be used inside modules. Export used in the context of: {$kind}.
export-only-allowed-in-top-level-scope = The 'export' keyword can only be used in the top level scope of a module.
import-requires-items-to-be-imported =
  The 'import' keyword requires either a scoped namespace or list of import items.
  help: try using 'import * as module from "module.dm"' or 'import {"{"} item {"}"} from "module.dm"'  