create table  IF NOT ExIsTs "passkeys"(
                    id UUID PRIMARY KEY UNIQUE NOT NULL,
                    name TEXT NOT NULL CHECK(length(name) > 0),
                    credential_id BYTEA UNIQUE NOT NULL,
                    public_key BYTEA NOT NULL,
                    sign_count INT DEFAULT 0 NOT NULL,
                    aaguid UUID,
                    transports TEXT[],
                    is_backed_up BOOLEAN DEFAULT FALSE,
                    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
                    lasted_used_at TIMESTAMPTZ,
                    _reserved BYTEA NULL,
                    metadata BYTEA NULL);

ALTER TABLE "passkeys" ADD COLUMN IF NOT EXISTs id UUID;
-- ALTER TABLE "passkeys" ALTER COLUMN UUID TYPE UUID USING id::uuid;
-- UPDATE "passkeys" SET id = gen_random_uuid() where id IS NULL;
-- ALTER TABLE "passkeys" ALTER COLUMN ID SET NOT NULL;

-- ALTER TABLE "passkeys" ADD COLUMN "name" IF NOT EXISTS TEXT;
-- UPDATE "passkeys" SET "name" = "id"::text where name IS NULL;
-- ALTER TABLE "passkeys" ALTER COLUMN "name" SET NOT NULL;

-- ALTER TABLE "passkeys" ADD COLUMN IF NOT EXISTS public_key BYTEA;
-- -- if data is not bytea this wont work correctly so we let it error
-- ALTER TABLE "passkeys" ALTER COLUMN public_key TYPE BYTEA;
-- ALTER TABLE "passkeys" ALTER COLUMN public_key SET NOT NULL;

-- ALTER TABLE "passkeys" ADD COLUMN IF NOT EXISTS sign_count INT;
-- ALTER TABLE "passkeys" ALTER COLUMN "sign_count" TYPE INT USING "sign_count"::int;
-- UPDATE "passkeys" SET "sign_count" = 0 where sign_count IS NULL;

-- ALTER TABLE "passkeys" ADD COLUMN IF NOT EXISTS aaguid UUID;
-- ALTER TABLE "passkeys" ALTER COLUMN aaguid TYPE UUID using "aaguid"::uuid;
-- ALTER TABLE "passkeys" ALTER COLUMN "aaguid" SET NULL;

-- ALTER TABLE "passkeys" ADD COLUMN IF NOT exists "transports" TEXT[];
-- ALTER TABLE "passkeys" ALTER COLUMN "transports" TYPE TEXT[];
-- UPDATE "passkeys" SET "transports" = '{}'::TEXT[] where "transports" IS NULL;
-- ALTER TABLE "passkeys" ALTER COLUMN "transports" SET NULL;

-- ALTER TABLE "passkeys" ADD COLUMN IF NOT EXISTS "is_backed_up" BOOLEAN;
-- ALTER TABLE "passkeys" ALTER COLUMN "is_backed_up" TYPE BOOLEAN using "is_backed_up"::BOOLEAN;
-- UPDATE "passkeys" SET "is_backed_up" = FALSE where "is_backed_up" IS NULL;
-- ALTER TABLE "passkeys" ALTER COLUMN "is_backed_up" SET NOT NULL;

-- DO $$ BEGIN RAISE EXCEPTION 'FINISH PASSKEYS MIGRATION SCRIPT'; END; $$


-- DO $$
-- BEGIN
--     CREATE INDEX IF NOT EXISTS idx_passkeys_id_v7 ON passkeys(id);

--  IF NOT Exists( SELECT 1 from pg_class c JOIN pg_namespace N on n.oid = c.relnamespace
--     WHERE c.relnamespace = 'idx_passkeys_id_v7' AND n.nspname = 'public'
--  ) THEN RAISE EXCEPTION 'INDEX idx_passkeys_id_v7 was not created on table passkeys';
--  END IF;
--  RAISE NOTICE 'Table index verified for table passkeys'
-- END; $$


-- -- this function checks if column C exists in table T with type TY and checks its null constraint
-- CREATE OR REPLACE FUNCTION migrations_check_column_exact(
--   target_table TEXT,
--   target_column TEXT,
--   expected_type TEXT,
--   is_not_null BOOLEAN DEFAULT TRUE)
-- RETURNS VOID AS $$
-- DECLARE
--   actual_nullable TEXT := CASE WHEN is_not_null THEN 'NO' ELSE 'YES' END;
-- BEGIN
--  IF NOT EXISTS (
--     SELECT 1
--     FROM information_schema.columns
--     WHERE table_name = target_table
--     AND column_name = target_column
--     AND (data_type = expected_type OR udt_name = expected_type)
--     AND is_nullable = actual_nullable
--     -- just to cover the bases
--     DISTINCT
--     LIMIT 1 
--  )
--  THEN RAISE EXCEPTION 'Table "%" must have column "%" as % (NOT NULL %)',
--  target_table,target_column,expected_type,actual_nullable
-- END;
-- $$ LANGUAGE plpgsql;

-- -- perform the validation
-- DO $$
-- DECLARE target_table_name TEXT := 'passkeys'
-- BEGIN

    
--     IF NOT EXISTS(SELECT 1 from information_schema.tables where table_name = target_table DISTINCT LIMIT 1)
--     THEN RAISE EXCEPTION 'MIGRATION FAILED TO CREATE The required table Passkeys';

--     PERFORM migrations_check_column_exact(target_table,'id','uuid');
--     PERFORM migrations_check_column_exact(target_table,'name','text');
--     PERFORM migrations_check_column_exact(target_table,'credential_id','bytea');
--     PERFORM migrations_check_column_exact(target_table, 'sign_count','int');
--     PERFORM migrations_check_column_exact(target_table, 'sign_count');
--     PERFORM migrations_check_column_exact(target_table,'aguuid','uuid',FALSE);
--     PERFORM migrations_check_column_exact(target_table,'transports','ARRAY',FALSE);
--     PERFORM migrations_check_column_exact(target_table,'is_backed_up','boolean',FALSE);
--     PERFORM migrations_check_column_exact(target_column,'created_at','timestamptz');
--     PERFORM migrations_check_column_exact(target_column,'last_used_at','timestamptz',FALSE);
--     PERFORM migrations_check_column_exact(target_column,'_reserved','bytea',FALSE);
--     PERFORM migrations_check_column_exact(target_column,'metadata','bytea',FALSE);
-- END; $$
