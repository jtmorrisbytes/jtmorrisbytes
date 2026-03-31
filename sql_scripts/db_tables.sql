SELECT
"t"."table_catalog",
"t"."table_schema", 
"t"."table_name",
"t"."table_type",
"c"."column_name",
"c"."data_type",
"c"."udt_name",
CASE "c"."is_nullable" WHEN 'YES' THEN 1 ELSE 0 END as "is_nullable"
FROM information_schema.tables as "t"
JOIN information_schema.columns as "c"
ON "t"."table_name" = "c"."table_name"
AND "t"."table_schema" = current_schema()
AND "t"."table_type" = 'BASE TABLE'
ORDER BY "t"."table_name","c"."ordinal_position"