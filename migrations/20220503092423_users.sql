# VARCHAR is used to represent an ID in most cases.
CREATE TABLE IF NOT EXISTS users (
	id VARCHAR NOT NULL,
	username VARCHAR(32) NOT NULL,
	display_name VARCHAR(32),
        
	# Thanks Emre, Olivier, Sharp Eyes and Sham.
	social_credit INT NOT NULL DEFAULT 0, # All hail Xi Jinping
	email VARCHAR(256) NOT NULL,
	password VARCHAR(256) NOT NULL,
	salt VARCHAR(32) NOT NULL,
	status VARCHAR(256),
	bio TEXT, # Should we uhhhhhh, limit this?
	avatar VARCHAR NOT NULL DEFAULT '0',
	banner VARCHAR,
	badges INTEGER NOT NULL DEFAULT 0, # bitfield
	permissions INTEGER NOT NULL DEFAULT 0, # bitfield
	pubkey VARCHAR NOT NULL, # Not an ID (duh), blame olivier for this.
	PRIMARY KEY (id)
);
