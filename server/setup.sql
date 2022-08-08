CREATE TABLE kernels (id INTEGER,name VARCHAR,codename VARCHAR,company VARCHAR,
        kernel_version VARCHAR,last_updated VARCHAR,developer VARCHAR,
        download_link VARCHAR);

CREATE TABLE requests (id INTEGER,device_name VARCHAR,device_codename VARCHAR,
        current_kernel_version VARCHAR,requested_android_version INTEGER,
        kernel_source VARCHAR);

