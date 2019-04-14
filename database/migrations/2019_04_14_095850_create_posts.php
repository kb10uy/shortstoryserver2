<?php

use Illuminate\Support\Facades\Schema;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Database\Migrations\Migration;

class CreatePosts extends Migration
{
    /**
     * Run the migrations.
     */
    public function up()
    {
        Schema::create('posts', function (Blueprint $table) {
            $table->bigIncrements('id');
            $table->timestamps();
            $table->foreign('user_id')
                ->nullable()
                ->references('id')->on('users')
                ->onDelete('set null');
            $table->text('title')
                ->notNullable();
            $table->text('body')
                ->notNullable();
            $table->string('body_type', 16)
                ->notNullable()
                ->default('plain');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down()
    {
        Schema::dropIfExists('posts');
    }
}
