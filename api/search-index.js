var searchIndex = JSON.parse('{\
"rrr":{"doc":"","i":[[3,"Note","rrr","Represents a single note in a note row.",null,null],[3,"NoteRow","","Stores the notes that belong to a single row in a beat.",null,null],[3,"Beat","","Stores all of the note rows that represent a beat.",null,null],[3,"BpmChange","","Represents a BPM change in a chart.",null,null],[3,"CompiledChart","","A runtime efficient representation of a chart used by an …",null,null],[3,"Chart","","A space/memory efficient representation of a chart.",null,null],[3,"Settings","","Stores RRR settings to start charts with.",null,null],[12,"speed","","",0,null],[3,"RRR","","",null,null],[11,"new","","",1,[[]]],[11,"with_settings","","",1,[[["settings",3]]]],[11,"start_chart","","",1,[[["compiledchart",3]]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_owned","","",5,[[]]],[11,"clone_into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_owned","","",6,[[]]],[11,"clone_into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"clone","","",2,[[],["note",3]]],[11,"clone","","",3,[[],["noterow",3]]],[11,"clone","","",4,[[],["beat",3]]],[11,"clone","","",5,[[],["bpmchange",3]]],[11,"clone","","",6,[[],["compiledchart",3]]],[11,"clone","","",7,[[],["chart",3]]],[11,"clone","","",0,[[],["settings",3]]],[11,"default","","",2,[[],["note",3]]],[11,"default","","",3,[[],["noterow",3]]],[11,"default","","",4,[[],["beat",3]]],[11,"default","","",5,[[],["bpmchange",3]]],[11,"default","","",6,[[],["compiledchart",3]]],[11,"default","","",7,[[],["chart",3]]],[11,"default","","",0,[[]]],[11,"default","","",1,[[],["rrr",3]]],[11,"cmp","","",2,[[["note",3]],["ordering",4]]],[11,"cmp","","",3,[[["noterow",3]],["ordering",4]]],[11,"cmp","","",4,[[["beat",3]],["ordering",4]]],[11,"cmp","","",6,[[["compiledchart",3]],["ordering",4]]],[11,"eq","","",2,[[["note",3]]]],[11,"ne","","",2,[[["note",3]]]],[11,"eq","","",3,[[["noterow",3]]]],[11,"ne","","",3,[[["noterow",3]]]],[11,"eq","","",4,[[["beat",3]]]],[11,"ne","","",4,[[["beat",3]]]],[11,"eq","","",5,[[["bpmchange",3]]]],[11,"ne","","",5,[[["bpmchange",3]]]],[11,"eq","","",6,[[["compiledchart",3]]]],[11,"eq","","",7,[[["chart",3]]]],[11,"ne","","",7,[[["chart",3]]]],[11,"eq","","",0,[[["settings",3]]]],[11,"ne","","",0,[[["settings",3]]]],[11,"partial_cmp","","",2,[[["note",3]],[["option",4],["ordering",4]]]],[11,"lt","","",2,[[["note",3]]]],[11,"le","","",2,[[["note",3]]]],[11,"gt","","",2,[[["note",3]]]],[11,"ge","","",2,[[["note",3]]]],[11,"partial_cmp","","",3,[[["noterow",3]],[["option",4],["ordering",4]]]],[11,"lt","","",3,[[["noterow",3]]]],[11,"le","","",3,[[["noterow",3]]]],[11,"gt","","",3,[[["noterow",3]]]],[11,"ge","","",3,[[["noterow",3]]]],[11,"partial_cmp","","",4,[[["beat",3]],[["option",4],["ordering",4]]]],[11,"lt","","",4,[[["beat",3]]]],[11,"le","","",4,[[["beat",3]]]],[11,"gt","","",4,[[["beat",3]]]],[11,"ge","","",4,[[["beat",3]]]],[11,"partial_cmp","","",5,[[["bpmchange",3]],[["option",4],["ordering",4]]]],[11,"lt","","",5,[[["bpmchange",3]]]],[11,"le","","",5,[[["bpmchange",3]]]],[11,"gt","","",5,[[["bpmchange",3]]]],[11,"ge","","",5,[[["bpmchange",3]]]],[11,"partial_cmp","","",6,[[["compiledchart",3]],[["option",4],["ordering",4]]]],[11,"partial_cmp","","",7,[[["chart",3]],[["option",4],["ordering",4]]]],[11,"lt","","",7,[[["chart",3]]]],[11,"le","","",7,[[["chart",3]]]],[11,"gt","","",7,[[["chart",3]]]],[11,"ge","","",7,[[["chart",3]]]],[11,"partial_cmp","","",0,[[["settings",3]],[["option",4],["ordering",4]]]],[11,"lt","","",0,[[["settings",3]]]],[11,"le","","",0,[[["settings",3]]]],[11,"gt","","",0,[[["settings",3]]]],[11,"ge","","",0,[[["settings",3]]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","","",6,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"hash","","",2,[[]]],[11,"hash","","",3,[[]]],[11,"hash","","",4,[[]]],[11,"hash","","",6,[[]]],[11,"new","","",2,[[]]],[11,"new","","",3,[[]]],[11,"new","","",4,[[]]],[11,"new","","",5,[[]]],[11,"new","","",7,[[]]],[11,"compile","","",7,[[],["compiledchart",3]]],[11,"new","","",0,[[]]]],"p":[[3,"Settings"],[3,"RRR"],[3,"Note"],[3,"NoteRow"],[3,"Beat"],[3,"BpmChange"],[3,"CompiledChart"],[3,"Chart"]]},\
"rrr_gui":{"doc":"","i":[[4,"Msg","rrr_gui","",null,null],[13,"SpitMessage","","",0,null],[3,"Model","","",null,null],[12,"link","","",1,null],[5,"main","","",null,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"create","","",1,[[["componentlink",6]]]],[11,"update","","",1,[[],["shouldrender",6]]],[11,"change","","",1,[[],["shouldrender",6]]],[11,"view","","",1,[[],["html",6]]]],"p":[[4,"Msg"],[3,"Model"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);