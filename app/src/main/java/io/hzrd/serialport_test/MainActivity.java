package io.hzrd.serialport_test;

import android.os.Bundle;
import android.util.Log;

import androidx.activity.EdgeToEdge;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;

import io.hzrd.serialport.SerialPort;

public class MainActivity extends AppCompatActivity {
    private SerialPort mSerialPort;
    private static final String TAG = "MainActivity";
    private static final char[] HEX_ARRAY = "0123456789ABCDEF".toCharArray();

    public static String toHex(byte[] bytes) {
        char[] hexChars = new char[bytes.length * 2];
        for (int j = 0; j < bytes.length; j++) {
            int v = bytes[j] & 0xFF;
            hexChars[j * 2] = HEX_ARRAY[v >>> 4];
            hexChars[j * 2 + 1] = HEX_ARRAY[v & 0x0F];
        }
        return new String(hexChars);
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        EdgeToEdge.enable(this);
        setContentView(R.layout.activity_main);
        ViewCompat.setOnApplyWindowInsetsListener(findViewById(R.id.main), (v, insets) -> {
            Insets systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars());
            v.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom);
            return insets;
        });

        try {
            System.loadLibrary("serialport");
        } catch (UnsatisfiedLinkError e) {
            Log.e(TAG, "loadLibrary: " + e);
            return;
        }
        String path = "/dev/ttyS0";
        Log.e(TAG, "path: " + path);
        mSerialPort = new SerialPort(path, 9600, 1);
        mSerialPort.open();
        byte[] sync = new byte[] { /* you're test bytes here... */ };
        Thread thread = new Thread() {
            @Override
            public void run() {
                try {
                    while (true) {
                        Log.e(TAG, "write: " + toHex(sync));
                        mSerialPort.write(sync);
                        sleep(1000);
                        Log.e(TAG, "read: " + toHex(mSerialPort.read(6)));
                    }
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            }
        };
        thread.start();
    }
}
