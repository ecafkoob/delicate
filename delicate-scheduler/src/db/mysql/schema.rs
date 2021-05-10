table! {
    /// Representation of the `executor_group` table.
    ///
    /// (Automatically generated by Diesel.)
    executor_group (id) {
        /// The `id` column of the `executor_group` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `name` column of the `executor_group` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `description` column of the `executor_group` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Varchar,
        /// The `tag` column of the `executor_group` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        tag -> Varchar,
        /// The `status` column of the `executor_group` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Smallint,
        /// The `created_time` column of the `executor_group` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_time -> Timestamp,
        /// The `deleted_time` column of the `executor_group` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    /// Representation of the `executor_processor` table.
    ///
    /// (Automatically generated by Diesel.)
    executor_processor (id) {
        /// The `id` column of the `executor_processor` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `name` column of the `executor_processor` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `host` column of the `executor_processor` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        host -> Varchar,
        /// The `port` column of the `executor_processor` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        port -> Smallint,
        /// The `description` column of the `executor_processor` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Varchar,
        /// The `tag` column of the `executor_processor` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        tag -> Varchar,
        /// The `status` column of the `executor_processor` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Smallint,
        /// The `created_time` column of the `executor_processor` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_time -> Timestamp,
        /// The `deleted_time` column of the `executor_processor` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    /// Representation of the `executor_processor_group` table.
    ///
    /// (Automatically generated by Diesel.)
    executor_processor_group (id) {
        /// The `id` column of the `executor_processor_group` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `name` column of the `executor_processor_group` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `group_id` column of the `executor_processor_group` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        group_id -> Bigint,
        /// The `executor_id` column of the `executor_processor_group` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        executor_id -> Bigint,
        /// The `weight` column of the `executor_processor_group` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        weight -> Smallint,
        /// The `status` column of the `executor_processor_group` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Smallint,
        /// The `created_time` column of the `executor_processor_group` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_time -> Timestamp,
        /// The `deleted_time` column of the `executor_processor_group` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    /// Representation of the `task` table.
    ///
    /// (Automatically generated by Diesel.)
    task (id) {
        /// The `id` column of the `task` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `name` column of the `task` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `description` column of the `task` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Varchar,
        /// The `command` column of the `task` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        command -> Varchar,
        /// The `frequency` column of the `task` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        frequency -> Varchar,
        /// The `cron_expression` column of the `task` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        cron_expression -> Varchar,
        /// The `timeout` column of the `task` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        timeout -> Smallint,
        /// The `retry_times` column of the `task` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        retry_times -> Smallint,
        /// The `retry_interval` column of the `task` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        retry_interval -> Smallint,
        /// The `maximun_parallel_runable_num` column of the `task` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        maximun_parallel_runable_num -> Smallint,
        /// The `tag` column of the `task` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        tag -> Varchar,
        /// The `status` column of the `task` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Smallint,
        /// The `created_time` column of the `task` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_time -> Timestamp,
        /// The `deleted_time` column of the `task` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    /// Representation of the `task_bind` table.
    ///
    /// (Automatically generated by Diesel.)
    task_bind (id) {
        /// The `id` column of the `task_bind` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `task_id` column of the `task_bind` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        task_id -> Bigint,
        /// The `bind_id` column of the `task_bind` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        bind_id -> Bigint,
        /// The `created_time` column of the `task_bind` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_time -> Timestamp,
    }
}

table! {
    /// Representation of the `task_log` table.
    ///
    /// (Automatically generated by Diesel.)
    task_log (id) {
        /// The `id` column of the `task_log` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `task_id` column of the `task_log` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        task_id -> Bigint,
        /// The `record_id` column of the `task_log` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        record_id -> Bigint,
        /// The `name` column of the `task_log` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `description` column of the `task_log` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Varchar,
        /// The `command` column of the `task_log` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        command -> Varchar,
        /// The `frequency` column of the `task_log` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        frequency -> Varchar,
        /// The `cron_expression` column of the `task_log` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        cron_expression -> Varchar,
        /// The `maximun_parallel_runable_num` column of the `task_log` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        maximun_parallel_runable_num -> Smallint,
        /// The `tag` column of the `task_log` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        tag -> Varchar,
        /// The `status` column of the `task_log` table.
        ///
        /// Its SQL type is `Smallint`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Smallint,
        /// The `created_time` column of the `task_log` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_time -> Timestamp,
        /// The `executor_processor_id` column of the `task_log` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        executor_processor_id -> Bigint,
        /// The `executor_processor_name` column of the `task_log` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        executor_processor_name -> Varchar,
        /// The `executor_processor_host` column of the `task_log` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        executor_processor_host -> Bigint,
    }
}

table! {
    /// Representation of the `task_log_extend` table.
    ///
    /// (Automatically generated by Diesel.)
    task_log_extend (id) {
        /// The `id` column of the `task_log_extend` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `stdout` column of the `task_log_extend` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        stdout -> Text,
        /// The `stderr` column of the `task_log_extend` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        stderr -> Text,
    }
}

table! {
    /// Representation of the `user` table.
    ///
    /// (Automatically generated by Diesel.)
    user (id) {
        /// The `id` column of the `user` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `user_name` column of the `user` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        user_name -> Varchar,
        /// The `nick_name` column of the `user` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        nick_name -> Varchar,
        /// The `mobile` column of the `user` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        mobile -> Varchar,
        /// The `email` column of the `user` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Varchar,
        /// The `face` column of the `user` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        face -> Varchar,
        /// The `status` column of the `user` table.
        ///
        /// Its SQL type is `Tinyint`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Tinyint,
        /// The `created_time` column of the `user` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_time -> Timestamp,
        /// The `updated_time` column of the `user` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_time -> Timestamp,
    }
}

table! {
    /// Representation of the `user_auth` table.
    ///
    /// (Automatically generated by Diesel.)
    user_auth (id) {
        /// The `id` column of the `user_auth` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `user_id` column of the `user_auth` table.
        ///
        /// Its SQL type is `Unsigned<Bigint>`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Unsigned<Bigint>,
        /// The `identity_type` column of the `user_auth` table.
        ///
        /// Its SQL type is `Unsigned<Tinyint>`.
        ///
        /// (Automatically generated by Diesel.)
        identity_type -> Unsigned<Tinyint>,
        /// The `identifier` column of the `user_auth` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        identifier -> Varchar,
        /// The `certificate` column of the `user_auth` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        certificate -> Varchar,
        /// The `status` column of the `user_auth` table.
        ///
        /// Its SQL type is `Tinyint`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Tinyint,
        /// The `created_time` column of the `user_auth` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_time -> Timestamp,
        /// The `updated_time` column of the `user_auth` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_time -> Timestamp,
    }
}

table! {
    /// Representation of the `user_info_update` table.
    ///
    /// (Automatically generated by Diesel.)
    user_info_update (id) {
        /// The `id` column of the `user_info_update` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `user_id` column of the `user_info_update` table.
        ///
        /// Its SQL type is `Unsigned<Bigint>`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Unsigned<Bigint>,
        /// The `attribute_name` column of the `user_info_update` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        attribute_name -> Varchar,
        /// The `attribute_old_val` column of the `user_info_update` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        attribute_old_val -> Varchar,
        /// The `attribute_new_val` column of the `user_info_update` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        attribute_new_val -> Varchar,
        /// The `updated_time` column of the `user_info_update` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_time -> Timestamp,
    }
}

table! {
    /// Representation of the `user_login_log` table.
    ///
    /// (Automatically generated by Diesel.)
    user_login_log (id) {
        /// The `id` column of the `user_login_log` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `user_id` column of the `user_login_log` table.
        ///
        /// Its SQL type is `Unsigned<Bigint>`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Unsigned<Bigint>,
        /// The `login_type` column of the `user_login_log` table.
        ///
        /// Its SQL type is `Unsigned<Tinyint>`.
        ///
        /// (Automatically generated by Diesel.)
        login_type -> Unsigned<Tinyint>,
        /// The `command` column of the `user_login_log` table.
        ///
        /// Its SQL type is `Unsigned<Tinyint>`.
        ///
        /// (Automatically generated by Diesel.)
        command -> Unsigned<Tinyint>,
        /// The `lastip` column of the `user_login_log` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        lastip -> Varchar,
        /// The `created_time` column of the `user_login_log` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_time -> Timestamp,
    }
}

table! {
    /// Representation of the `user_register_log` table.
    ///
    /// (Automatically generated by Diesel.)
    user_register_log (id) {
        /// The `id` column of the `user_register_log` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `user_id` column of the `user_register_log` table.
        ///
        /// Its SQL type is `Unsigned<Bigint>`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Unsigned<Bigint>,
        /// The `register_method` column of the `user_register_log` table.
        ///
        /// Its SQL type is `Unsigned<Tinyint>`.
        ///
        /// (Automatically generated by Diesel.)
        register_method -> Unsigned<Tinyint>,
        /// The `register_time` column of the `user_register_log` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        register_time -> Timestamp,
        /// The `register_ip` column of the `user_register_log` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        register_ip -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    executor_group,
    executor_processor,
    executor_processor_group,
    task,
    task_bind,
    task_log,
    task_log_extend,
    user,
    user_auth,
    user_info_update,
    user_login_log,
    user_register_log,
);
