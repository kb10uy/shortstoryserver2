<?php

namespace App\Rules;

use Exception;
use GuzzleHttp\Client;
use Illuminate\Contracts\Validation\Rule;
use Log;

class Recaptcha3 implements Rule
{
    /** @var Client client */
    private $client;

    /** @var float bot と判定する score の閾値 */
    private $threshold;

    /**
     * Create a new rule instance.
     *
     * @param float $threshold 閾値
     */
    public function __construct(float $threshold)
    {
        $this->client = new Client();
        $this->threshold = $threshold;
    }

    /**
     * Determine if the validation rule passes.
     *
     * @param string $attribute
     * @param mixed  $value
     *
     * @return bool
     */
    public function passes($attribute, $value)
    {
        // もらったトークンで検証
        try {
            $response = $this->client->post('https://www.google.com/recaptcha/api/siteverify', [
                'form_params' => [
                    'secret' => env('RECAPTCHA3_SECRET_KEY'),
                    'response' => $value,
                ],
            ]);
            $data = json_decode((string) $response->getBody(), true);

            return $data['success'] && $data['score'] >= $this->threshold;
        } catch (Exception $ex) {
            Log::error("Failed to verify reCAPTCHA v3 token: {$ex->getMessage()}");

            return false;
        }
    }

    /**
     * Get the validation error message.
     *
     * @return string
     */
    public function message()
    {
        return 'The reCAPTCHA verification failed.';
    }
}
