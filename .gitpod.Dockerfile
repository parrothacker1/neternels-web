FROM gitpod/workspace-postgres

EXPOSE 5432

CMD pg_start;pg_ctl status
