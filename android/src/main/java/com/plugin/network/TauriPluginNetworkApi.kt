package com.plugin.network;

import android.app.Activity;
import app.tauri.annotation.TauriPlugin;
import app.tauri.plugin.Plugin;

@TauriPlugin()
class TauriPluginNetworkApi(private val activity: Activity): Plugin(activity) {
    
}