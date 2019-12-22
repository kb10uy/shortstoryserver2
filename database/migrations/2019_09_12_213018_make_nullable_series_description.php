<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

class MakeNullableSeriesDescription extends Migration
{
    /**
     * Run the migrations.
     */
    public function up()
    {
        Schema::table('series', function (Blueprint $table) {
            $table->string('description', 500)->nullable()->change();
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down()
    {
        Schema::table('series', function (Blueprint $table) {
            $table->string('description', 500)->notNullable()->change();
        });
    }
}
