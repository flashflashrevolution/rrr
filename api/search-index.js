var searchIndex = JSON.parse('{\
"rrr":{"doc":"","i":[[3,"Note","rrr","Represents a single note in a note row.",null,null],[3,"NoteRow","","Stores the notes that belong to a single row in a beat.",null,null],[3,"Beat","","Stores all of the note rows that represent a beat.",null,null],[3,"BpmChange","","Represents a BPM change in a chart.",null,null],[3,"CompiledChart","","A runtime efficient representation of a chart used by an…",null,null],[3,"Chart","","A space/memory efficient representation of a chart.",null,null],[3,"Settings","","Stores RRR settings to play charts with.",null,null],[3,"SettingsBuilder","","A builder for a Settings instance.",null,null],[3,"RRR","","",null,null],[11,"new","","",0,[[]]],[11,"new","","",1,[[["rational",6]]]],[11,"new","","",2,[[]]],[11,"new","","",3,[[["rational",6]]]],[11,"new","","",4,[[]]],[11,"new","","",5,[[]]],[11,"build","","",5,[[],["settings",3]]],[11,"new","","",6,[[]]],[11,"with_settings","","",6,[[["settings",3]]]],[11,"start_chart","","",6,[[["compiledchart",3]]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"to_owned","","",8,[[]]],[11,"clone_into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_owned","","",5,[[]]],[11,"clone_into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",7,[[["chart",3]]]],[11,"clone","","",0,[[],["note",3]]],[11,"clone","","",1,[[],["noterow",3]]],[11,"clone","","",2,[[],["beat",3]]],[11,"clone","","",3,[[],["bpmchange",3]]],[11,"clone","","",7,[[],["compiledchart",3]]],[11,"clone","","",4,[[],["chart",3]]],[11,"clone","","",8,[[],["settings",3]]],[11,"clone","","",5,[[],["settingsbuilder",3]]],[11,"default","","",0,[[],["note",3]]],[11,"default","","",1,[[]]],[11,"default","","",2,[[],["beat",3]]],[11,"default","","",3,[[]]],[11,"default","","",7,[[],["compiledchart",3]]],[11,"default","","",4,[[],["chart",3]]],[11,"default","","",8,[[],["settings",3]]],[11,"default","","",5,[[],["settingsbuilder",3]]],[11,"default","","",6,[[],["rrr",3]]],[11,"cmp","","",0,[[["note",3]],["ordering",4]]],[11,"cmp","","",1,[[["noterow",3]],["ordering",4]]],[11,"cmp","","",2,[[["beat",3]],["ordering",4]]],[11,"cmp","","",3,[[["bpmchange",3]],["ordering",4]]],[11,"cmp","","",7,[[["compiledchart",3]],["ordering",4]]],[11,"cmp","","",4,[[["chart",3]],["ordering",4]]],[11,"cmp","","",8,[[["settings",3]],["ordering",4]]],[11,"cmp","","",5,[[["settingsbuilder",3]],["ordering",4]]],[11,"eq","","",0,[[["note",3]]]],[11,"ne","","",0,[[["note",3]]]],[11,"eq","","",1,[[["noterow",3]]]],[11,"ne","","",1,[[["noterow",3]]]],[11,"eq","","",2,[[["beat",3]]]],[11,"ne","","",2,[[["beat",3]]]],[11,"eq","","",3,[[["bpmchange",3]]]],[11,"ne","","",3,[[["bpmchange",3]]]],[11,"eq","","",7,[[["compiledchart",3]]]],[11,"eq","","",4,[[["chart",3]]]],[11,"ne","","",4,[[["chart",3]]]],[11,"eq","","",8,[[["settings",3]]]],[11,"eq","","",5,[[["settingsbuilder",3]]]],[11,"partial_cmp","","",0,[[["note",3]],[["ordering",4],["option",4]]]],[11,"lt","","",0,[[["note",3]]]],[11,"le","","",0,[[["note",3]]]],[11,"gt","","",0,[[["note",3]]]],[11,"ge","","",0,[[["note",3]]]],[11,"partial_cmp","","",1,[[["noterow",3]],[["ordering",4],["option",4]]]],[11,"lt","","",1,[[["noterow",3]]]],[11,"le","","",1,[[["noterow",3]]]],[11,"gt","","",1,[[["noterow",3]]]],[11,"ge","","",1,[[["noterow",3]]]],[11,"partial_cmp","","",2,[[["beat",3]],[["ordering",4],["option",4]]]],[11,"lt","","",2,[[["beat",3]]]],[11,"le","","",2,[[["beat",3]]]],[11,"gt","","",2,[[["beat",3]]]],[11,"ge","","",2,[[["beat",3]]]],[11,"partial_cmp","","",3,[[["bpmchange",3]],[["ordering",4],["option",4]]]],[11,"lt","","",3,[[["bpmchange",3]]]],[11,"le","","",3,[[["bpmchange",3]]]],[11,"gt","","",3,[[["bpmchange",3]]]],[11,"ge","","",3,[[["bpmchange",3]]]],[11,"partial_cmp","","",7,[[["compiledchart",3]],[["ordering",4],["option",4]]]],[11,"partial_cmp","","",4,[[["chart",3]],[["ordering",4],["option",4]]]],[11,"lt","","",4,[[["chart",3]]]],[11,"le","","",4,[[["chart",3]]]],[11,"gt","","",4,[[["chart",3]]]],[11,"ge","","",4,[[["chart",3]]]],[11,"partial_cmp","","",8,[[["settings",3]],[["ordering",4],["option",4]]]],[11,"partial_cmp","","",5,[[["settingsbuilder",3]],[["ordering",4],["option",4]]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","","",6,[[["formatter",3]],["result",6]]],[11,"hash","","",0,[[]]],[11,"hash","","",1,[[]]],[11,"hash","","",2,[[]]],[11,"hash","","",3,[[]]],[11,"hash","","",7,[[]]],[11,"hash","","",4,[[]]],[11,"hash","","",8,[[]]],[11,"hash","","",5,[[]]],[11,"serialize","","",0,[[],["result",4]]],[11,"serialize","","",1,[[],["result",4]]],[11,"serialize","","",2,[[],["result",4]]],[11,"serialize","","",3,[[],["result",4]]],[11,"serialize","","",7,[[],["result",4]]],[11,"serialize","","",4,[[],["result",4]]],[11,"serialize","","",8,[[],["result",4]]],[11,"serialize","","",5,[[],["result",4]]],[11,"deserialize","","",0,[[],["result",4]]],[11,"deserialize","","",1,[[],["result",4]]],[11,"deserialize","","",2,[[],["result",4]]],[11,"deserialize","","",3,[[],["result",4]]],[11,"deserialize","","",7,[[],["result",4]]],[11,"deserialize","","",4,[[],["result",4]]],[11,"deserialize","","",8,[[],["result",4]]],[11,"deserialize","","",5,[[],["result",4]]]],"p":[[3,"Note"],[3,"NoteRow"],[3,"Beat"],[3,"BpmChange"],[3,"Chart"],[3,"SettingsBuilder"],[3,"RRR"],[3,"CompiledChart"],[3,"Settings"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);