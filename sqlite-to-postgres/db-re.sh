#!/bin/bash
cp /vagrant/databases/mini-x.db ./test.db
sqlite3 test.db < fix.sql
pgloader test.load