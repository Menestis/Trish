CREATE TABLE IF NOT EXISTS games
(
    guild                BIGINT UNSIGNED NOT NULL,
    announcement_channel BIGINT UNSIGNED NOT NULL,
    announcement_msg     BIGINT UNSIGNED NOT NULL,
    channel              BIGINT UNSIGNED NOT NULL,
    role                 BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (guild, announcement_channel, announcement_msg)
);