(function(global) {
  'use strict';

  var CONFIG = {
    clientId: '__GOOGLE_CLIENT_ID__',
    scopes: 'https://www.googleapis.com/auth/drive.appdata',
    fileName: 'people_modeler_backup.json',
    tokenKey: 'pm_gdrive_token',
    connectedKey: 'pm_gdrive_connected',
  };

  var tokenClient = null;
  var accessToken = null;
  var ready = false;

  function isConfigured() {
    return CONFIG.clientId.indexOf('__') !== 0;
  }

  function init() {
    if (ready) return true;
    if (!isConfigured()) return false;
    if (typeof global.google === 'undefined' || !global.google.accounts || !global.google.accounts.oauth2) return false;

    accessToken = localStorage.getItem(CONFIG.tokenKey);
    if (accessToken) localStorage.setItem(CONFIG.connectedKey, '1');

    tokenClient = global.google.accounts.oauth2.initTokenClient({
      client_id: CONFIG.clientId,
      scope: CONFIG.scopes,
      callback: function(resp) {
        if (resp.access_token) {
          accessToken = resp.access_token;
          localStorage.setItem(CONFIG.tokenKey, resp.access_token);
          localStorage.setItem(CONFIG.connectedKey, '1');
          global.dispatchEvent(new global.CustomEvent('gdrive:connected'));
        }
      },
    });
    ready = true;
    return true;
  }

  function isConnected() {
    return ready && !!accessToken;
  }

  function requestToken(showUi) {
    return new Promise(function(resolve, reject) {
      if (!tokenClient) {
        reject(new Error('GIS library not loaded. Check network / ad-blocker.'));
        return;
      }
      tokenClient.callback = function(resp) {
        if (resp.access_token) {
          accessToken = resp.access_token;
          localStorage.setItem(CONFIG.tokenKey, resp.access_token);
          localStorage.setItem(CONFIG.connectedKey, '1');
          resolve(resp);
        } else {
          reject(resp.error || new Error('OAuth failed'));
        }
      };
      if (showUi === false) {
        tokenClient.requestAccessToken({ prompt: '' });
      } else {
        tokenClient.requestAccessToken();
      }
    });
  }

  async function signIn() {
    if (!init()) throw new Error('GDrive not available — check client ID and GIS library');
    return requestToken(true);
  }

  function getAuthHeader() {
    return 'Bearer ' + accessToken;
  }

  async function callApi(path, opts) {
    if (!accessToken) throw new Error('Not authenticated');
    opts = opts || {};
    var method = opts.method || 'GET';
    var body = opts.body;
    var extraHeaders = opts.headers || {};

    var res = await global.fetch('https://www.googleapis.com/drive/v3/' + path, {
      method: method,
      headers: Object.assign({
        Authorization: getAuthHeader(),
        'Content-Type': 'application/json',
      }, extraHeaders),
      body: body ? (typeof body === 'string' ? body : JSON.stringify(body)) : undefined,
    });

    if (res.status === 401) {
      accessToken = null;
      localStorage.removeItem(CONFIG.tokenKey);
      localStorage.removeItem(CONFIG.connectedKey);
      try {
        await requestToken(false);
      } catch (e) {
        throw new Error('Token expired');
      }
      return callApi(path, opts);
    }

    return res;
  }

  async function findOrCreateFile() {
    var res = await callApi("files?q=name='" + CONFIG.fileName + "'+and+'appDataFolder'+in+parents&spaces=appDataFolder&fields=files(id,name,modifiedTime)");
    var data = await res.json();
    if (data.files && data.files.length > 0) return data.files[0];

    var createRes = await callApi('files', {
      method: 'POST',
      body: { name: CONFIG.fileName, parents: ['appDataFolder'], mimeType: 'application/json' },
    });
    return createRes.json();
  }

  async function backup(persons) {
    var file = await findOrCreateFile();
    var payload = {
      version: 1,
      timestamp: Date.now(),
      persons: persons,
    };
    var body = JSON.stringify(payload);

    var res = await global.fetch(
      'https://www.googleapis.com/upload/drive/v3/files/' + file.id + '?uploadType=media',
      {
        method: 'PATCH',
        headers: {
          Authorization: getAuthHeader(),
          'Content-Type': 'application/json',
        },
        body: body,
      }
    );

    if (res.status === 401) {
      accessToken = null;
      localStorage.removeItem(CONFIG.tokenKey);
      localStorage.removeItem(CONFIG.connectedKey);
      throw new Error('Token expired');
    }
    return res.ok;
  }

  async function restore() {
    var file = await findOrCreateFile();
    var res = await global.fetch(
      'https://www.googleapis.com/drive/v3/files/' + file.id + '?alt=media',
      { headers: { Authorization: getAuthHeader() } }
    );
    if (res.ok) {
      var data = await res.json();
      return data.persons || [];
    }
    if (res.status === 401) {
      accessToken = null;
      localStorage.removeItem(CONFIG.tokenKey);
      localStorage.removeItem(CONFIG.connectedKey);
      throw new Error('Token expired');
    }
    return null;
  }

  async function getBackupInfo() {
    var file = await findOrCreateFile();
    var res = await callApi('files/' + file.id + '?fields=modifiedTime,size');
    return res.json();
  }

  function signOut() {
    if (accessToken && global.google && global.google.accounts && global.google.accounts.oauth2) {
      try { global.google.accounts.oauth2.revoke(accessToken, function() {}); } catch(e) {}
    }
    accessToken = null;
    localStorage.removeItem(CONFIG.tokenKey);
    localStorage.removeItem(CONFIG.connectedKey);
    global.dispatchEvent(new global.CustomEvent('gdrive:disconnected'));
  }

  var autoBackupTimer = null;

  function scheduleBackup(persons) {
    if (autoBackupTimer) clearTimeout(autoBackupTimer);
    autoBackupTimer = setTimeout(function() {
      backup(persons).catch(function(err) {
        console.warn('GDrive backup failed:', err);
      });
    }, 2000);
  }

  global.GDrive = {
    init: init,
    isConfigured: isConfigured,
    isConnected: isConnected,
    signIn: signIn,
    signOut: signOut,
    backup: backup,
    restore: restore,
    getBackupInfo: getBackupInfo,
    scheduleBackup: scheduleBackup,
  };
})(window);
