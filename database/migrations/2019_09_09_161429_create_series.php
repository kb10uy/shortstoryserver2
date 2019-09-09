<?php

use Illuminate\Support\Facades\Schema;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Database\Migrations\Migration;

class CreateSeries extends Migration
{
    /**
     * Run the migrations.
     *
     * @return void
     */
    public function up()
    {
        Schema::create('series', function (Blueprint $table) {
            $table->bigIncrements('id');
            $table->string('title', 128)->notNullable();
            $table->string('description', 500);
            $table->bigInteger('user_id')->notNullable()->references('id')->on('users')->onDelete('cascade');
            $table->timestamps();
        });

        Schema::create('series_posts', function (Blueprint $table) {
            $table->bigIncrements('id');
            $table->bigInteger('series_id')->notNullable()->references('id')->on('series')->onDelete('cascade');
            $table->bigInteger('post_id')->notNullable()->references('id')->on('posts')->onDelete('restrict');
            $table->integer('order')->notNullable();
            $table->timestamps();

            $table->unique(['series_id', 'post_id']);
            // これは遅延と一緒に下で設定する
            // $table->unique(['series_id', 'order']);
        });

        DB::raw('ALTER TABLE posts_series ADD CONSTRAINT post_order_uniqueness UNIQUE (series_id, order) DEFERRABLE;');
    }

    /**
     * Reverse the migrations.
     *
     * @return void
     */
    public function down()
    {
        Schema::dropIfExists('series');
        Schema::dropIfExists('series_posts');
    }
}
