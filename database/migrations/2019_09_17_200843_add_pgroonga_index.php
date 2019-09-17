<?php

use Illuminate\Support\Facades\Schema;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Database\Migrations\Migration;

class AddPgroongaIndex extends Migration
{
    /**
     * Run the migrations.
     *
     * @return void
     */
    public function up()
    {
        DB::statement('CREATE EXTENSION pgroonga;');
        DB::statement('CREATE INDEX pgroonga_full_text ON posts USING pgroonga (body);');
    }

    /**
     * Reverse the migrations.
     *
     * @return void
     */
    public function down()
    {
        DB::statement('DROP INDEX pgroonga_full_text ON posts;');
        DB::statement('DROP EXTENSION pgroonga;');
    }
}
