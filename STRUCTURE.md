# Stucture for the new re-write

## Modules

Modules will now have new structure:

`src/ui/modules/module_name`

`module_name/module.rs:` This will have the main functionality for the module like if it is a battery module this will have the code to get all info about battery with get_functions.

`module_name/module_box.rs:` This will hold the ui for the module in a box with a icon and a label. Can show either icon only, label only, both or none (idk why).

`module_name/module_updater.rs:` This will have the code to update the info for the module. This is also optional.
