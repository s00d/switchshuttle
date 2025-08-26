import { SwitchShuttleCommands } from './tauri-commands';
import { errorHandler } from './error-handler';

// Security types
export interface SecurityConfig {
  maxCommandLength: number;
  allowedCommands: string[];
  blockedCommands: string[];
  maxInputLength: number;
  suspiciousPatterns: RegExp[];
}

export interface SecurityValidation {
  isValid: boolean;
  errors: SecurityIssue[];
  warnings: SecurityIssue[];
  suggestions: string[];
}

export interface SecurityIssue {
  type: 'error' | 'warning';
  message: string;
  command?: string;
  index?: number;
  pattern?: string;
  severity?: 'low' | 'medium' | 'high' | 'critical';
}

export interface SuspiciousActivity {
  type: string;
  details: any;
  timestamp: Date;
  severity: 'low' | 'medium' | 'high';
}

export class SecurityManager {
  private static instance: SecurityManager;
  private config: SecurityConfig = {
    maxCommandLength: 1000,
    allowedCommands: [],
    blockedCommands: ['rm -rf /', 'format c:', 'del /s /q'],
    maxInputLength: 500,
    suspiciousPatterns: [
      /rm\s+-rf/i,
      /del\s+\/s/i,
      /format\s+c:/i,
      /shutdown/i,
      /halt/i,
      /reboot/i,
      /killall/i,
      /taskkill/i,
      /sudo\s+/i,
      /curl\s+/i,
      /wget\s+/i,
      /nc\s+/i,
      /ncat\s+/i,
      /scp\s+/i,
      /ftp\s+/i,
      /powershell/i,
      /chmod\s+777/i,
      /chown\s+/i,
      /dd\s+/i,
      /mkfs/i,
      /netcat/i,
      /base64\s+-d/i,
      /openssl\s+/i,
    ],
  };

  private suspiciousActivities: SuspiciousActivity[] = [];
  private maxActivities = 50;

  private constructor() {}

  static getInstance(): SecurityManager {
    if (!SecurityManager.instance) {
      SecurityManager.instance = new SecurityManager();
    }
    return SecurityManager.instance;
  }

  /**
   * Updates configuration from Tauri settings
   */
  async updateConfigFromSettings(): Promise<void> {
    try {
      const settings = await SwitchShuttleCommands.get_security_settings();
      console.log('[Security] Loading settings:', settings);
      
      this.config = {
        maxCommandLength: settings.max_command_length || 1000,
        allowedCommands: [],
        blockedCommands: settings.blocked_commands || [
          'rm -rf /',
          'format c:',
          'del /s /q',
        ],
        maxInputLength: settings.max_input_length || 500,
        suspiciousPatterns: (settings.suspicious_patterns || []).map(
          pattern => new RegExp(pattern, 'i')
        ),
      };
      
      console.log('[Security] Updated config:', {
        maxCommandLength: this.config.maxCommandLength,
        blockedCommands: this.config.blockedCommands,
        maxInputLength: this.config.maxInputLength,
        suspiciousPatterns: this.config.suspiciousPatterns.map(p => p.source),
      });
    } catch (error) {
      console.warn('Failed to load security settings, using defaults:', error);
    }
  }

  /**
   * Checks if security checks are enabled
   */
  async isSecurityEnabled(): Promise<boolean> {
    try {
      const settings = await SwitchShuttleCommands.get_security_settings();
      return settings.enable_security_checks !== false;
    } catch (error) {
      console.warn(
        'Failed to check security settings, assuming enabled:',
        error
      );
      return true;
    }
  }

  /**
   * Deeply analyzes a single command or an array of commands for security issues
   */
  async analyzeCommands(
    commands: string | string[]
  ): Promise<SecurityValidation> {
    // Check if security is enabled
    if (!(await this.isSecurityEnabled())) {
      return {
        isValid: true,
        errors: [],
        warnings: [],
        suggestions: [],
      };
    }

    // Update config from settings
    await this.updateConfigFromSettings();

    const errors: SecurityIssue[] = [];
    const warnings: SecurityIssue[] = [];
    const suggestions: string[] = [];
    const commandList = Array.isArray(commands) ? commands : [commands];

    for (let i = 0; i < commandList.length; i++) {
      const command = commandList[i];
      // Split by &&, ||, ; to analyze each subcommand
      const subcommands = this.splitMultiCommand(command);
      for (const sub of subcommands) {
        // Length check
        if (sub.length > this.config.maxCommandLength) {
          errors.push({
            type: 'error',
            message: `Command is too long (max ${this.config.maxCommandLength} chars)`,
            command: sub,
            index: i,
            severity: 'high',
          });
        }

        // Blocked commands check
        for (const blocked of this.config.blockedCommands) {
          if (sub.toLowerCase().includes(blocked.toLowerCase())) {
            errors.push({
              type: 'error',
              message: `Blocked command pattern detected: ${blocked}`,
              command: sub,
              index: i,
              pattern: blocked,
              severity: 'critical',
            });
          }
        }

        // Suspicious patterns check
        for (const pattern of this.config.suspiciousPatterns) {
          if (pattern.test(sub)) {
            warnings.push({
              type: 'warning',
              message: `Suspicious pattern detected: ${pattern.source}`,
              command: sub,
              index: i,
              pattern: pattern.source,
              severity: 'high',
            });
          }
        }

        // Dangerous characters
        const dangerousChars = [';', '|', '&', '>', '<', '`', '$', '(', ')'];
        const foundChars = dangerousChars.filter(char => sub.includes(char));
        if (foundChars.length > 0) {
          warnings.push({
            type: 'warning',
            message: `Suspicious characters: ${foundChars.join(', ')}`,
            command: sub,
            index: i,
            severity: 'medium',
          });
        }

        // Multiple commands
        if (sub.includes('&&') || sub.includes('||') || sub.includes(';')) {
          warnings.push({
            type: 'warning',
            message: 'Multiple commands detected',
            command: sub,
            index: i,
            severity: 'medium',
          });
        }

        // Sudo/root escalation
        if (/\bsudo\b|\bsu\b|\bdoas\b/.test(sub)) {
          warnings.push({
            type: 'warning',
            message: 'Privilege escalation detected (sudo/su/doas)',
            command: sub,
            index: i,
            severity: 'high',
          });
        }

        // Network operations
        if (/\b(curl|wget|ftp|scp|nc|ncat|netcat)\b/.test(sub)) {
          warnings.push({
            type: 'warning',
            message: 'Network operation detected',
            command: sub,
            index: i,
            severity: 'medium',
          });
        }

        // File system destructive actions
        if (/\brm\b|\bdel\b|\bformat\b|\bdd\b|\bmkfs\b/.test(sub)) {
          warnings.push({
            type: 'warning',
            message: 'Potentially destructive file system operation',
            command: sub,
            index: i,
            severity: 'high',
          });
        }

        // Suggestion: use absolute paths
        if (/\bcd\b/.test(sub) && !/\bcd\s+\//.test(sub)) {
          suggestions.push('Consider using absolute paths in cd commands');
        }
      }
    }

    return {
      isValid: errors.length === 0,
      errors,
      warnings,
      suggestions,
    };
  }

  /**
   * Splits a command string into subcommands by &&, ||, ;
   */
  splitMultiCommand(command: string): string[] {
    // Split by &&, ||, ; but keep quoted substrings together
    const result: string[] = [];
    let current = '';
    let inSingle = false;
    let inDouble = false;
    for (let i = 0; i < command.length; i++) {
      const char = command[i];
      if (char === "'" && !inDouble) inSingle = !inSingle;
      if (char === '"' && !inSingle) inDouble = !inDouble;
      if (
        !inSingle &&
        !inDouble &&
        (command.slice(i, i + 2) === '&&' || command.slice(i, i + 2) === '||')
      ) {
        if (current.trim()) result.push(current.trim());
        current = '';
        i++;
        continue;
      }
      if (!inSingle && !inDouble && char === ';') {
        if (current.trim()) result.push(current.trim());
        current = '';
        continue;
      }
      current += char;
    }
    if (current.trim()) result.push(current.trim());
    return result;
  }

  /**
   * Validates user input
   */
  validateInput(input: Record<string, string>): SecurityValidation {
    const errors: SecurityIssue[] = [];
    const warnings: SecurityIssue[] = [];
    const suggestions: string[] = [];

    for (const [key, value] of Object.entries(input)) {
      // Check length
      if (value.length > this.config.maxInputLength) {
        errors.push({
          type: 'error',
          message: `Field "${key}" is too long`,
          command: value,
          severity: 'high',
        });
      }

      // Check for dangerous characters
      const dangerousChars = [';', '|', '&', '>', '<', '`', '$'];
      const foundChars = dangerousChars.filter(char => value.includes(char));
      if (foundChars.length > 0) {
        errors.push({
          type: 'error',
          message: `Field "${key}" contains invalid characters: ${foundChars.join(', ')}`,
          command: value,
          severity: 'high',
        });
      }

      // Check for suspicious patterns
      const suspiciousPatterns = [
        /javascript:/i,
        /data:/i,
        /vbscript:/i,
        /on\w+\s*=/i,
      ];

      for (const pattern of suspiciousPatterns) {
        if (pattern.test(value)) {
          warnings.push({
            type: 'warning',
            message: `Field "${key}" contains suspicious content`,
            command: value,
            severity: 'medium',
          });
          break;
        }
      }
    }

    return {
      isValid: errors.length === 0,
      errors,
      warnings,
      suggestions,
    };
  }

  /**
   * Sanitizes command
   */
  sanitizeCommand(command: string): string {
    // Remove dangerous characters
    let sanitized = command
      .replace(/[;&|`$]/g, '')
      .replace(/[<>]/g, '')
      .trim();

    // Limit length
    if (sanitized.length > this.config.maxCommandLength) {
      sanitized = sanitized.substring(0, this.config.maxCommandLength);
    }

    return sanitized;
  }

  /**
   * Sanitizes user input
   */
  sanitizeInput(input: Record<string, string>): Record<string, string> {
    const sanitized: Record<string, string> = {};

    for (const [key, value] of Object.entries(input)) {
      let sanitizedValue = value
        .replace(/[;&|`$]/g, '')
        .replace(/[<>]/g, '')
        .trim();

      // Limit length
      if (sanitizedValue.length > this.config.maxInputLength) {
        sanitizedValue = sanitizedValue.substring(
          0,
          this.config.maxInputLength
        );
      }

      sanitized[key] = sanitizedValue;
    }

    return sanitized;
  }

  /**
   * Checks file access permissions
   */
  async checkFileAccess(
    filePath: string
  ): Promise<{ hasAccess: boolean; error?: string }> {
    try {
      // Check if file is in allowed directory
      const allowedPaths = [
        (globalThis as any).process?.env?.HOME,
        (globalThis as any).process?.env?.USERPROFILE,
        (globalThis as any).process?.env?.APPDATA,
      ].filter(Boolean);

      const isAllowed = allowedPaths.some(path =>
        filePath.startsWith(path || '')
      );

      if (!isAllowed) {
        await this.logSuspiciousActivity('file_access_denied', {
          filePath,
          allowedPaths,
        });
        return {
          hasAccess: false,
          error: 'File access denied',
        };
      }

      return { hasAccess: true };
    } catch (error) {
      await errorHandler.handleSecurityError(
        error as Error,
        'check_file_access'
      );
      return {
        hasAccess: false,
        error: `Access check error: ${error}`,
      };
    }
  }

  /**
   * Logs suspicious activity
   */
  async logSuspiciousActivity(activity: string, details?: any): Promise<void> {
    const suspiciousActivity: SuspiciousActivity = {
      type: activity,
      details,
      timestamp: new Date(),
      severity: this.determineActivitySeverity(activity),
    };

    this.suspiciousActivities.push(suspiciousActivity);

    // Limit the number of records
    if (this.suspiciousActivities.length > this.maxActivities) {
      this.suspiciousActivities = this.suspiciousActivities.slice(
        -this.maxActivities
      );
    }

    try {
      await SwitchShuttleCommands.show_warning_notification(
        'Suspicious Activity',
        `Suspicious activity detected: ${activity}`
      );

      console.warn('[Security] Suspicious activity detected:', {
        activity,
        details,
        timestamp: new Date().toISOString(),
        severity: suspiciousActivity.severity,
      });
    } catch (error) {
      console.error('[Security] Failed to log suspicious activity:', error);
    }
  }

  /**
   * Determines suspicious activity severity
   */
  private determineActivitySeverity(
    activity: string
  ): 'low' | 'medium' | 'high' {
    const highSeverityActivities = [
      'dangerous_pattern',
      'blocked_command',
      'file_access_denied',
      'privilege_escalation',
    ];

    const mediumSeverityActivities = [
      'suspicious_input',
      'multiple_commands',
      'suspicious_chars',
    ];

    if (highSeverityActivities.includes(activity)) {
      return 'high';
    }
    if (mediumSeverityActivities.includes(activity)) {
      return 'medium';
    }
    return 'low';
  }

  /**
   * Gets suspicious activity statistics
   */
  getSuspiciousActivityStats(): {
    total: number;
    bySeverity: Record<string, number>;
    byType: Record<string, number>;
  } {
    const bySeverity: Record<string, number> = {};
    const byType: Record<string, number> = {};

    this.suspiciousActivities.forEach(activity => {
      // Count by severity
      bySeverity[activity.severity] = (bySeverity[activity.severity] || 0) + 1;

      // Count by type
      byType[activity.type] = (byType[activity.type] || 0) + 1;
    });

    return {
      total: this.suspiciousActivities.length,
      bySeverity,
      byType,
    };
  }

  /**
   * Gets suspicious activities by type
   */
  getSuspiciousActivitiesByType(type: string): SuspiciousActivity[] {
    return this.suspiciousActivities.filter(activity => activity.type === type);
  }

  /**
   * Gets suspicious activities by severity
   */
  getSuspiciousActivitiesBySeverity(
    severity: 'low' | 'medium' | 'high'
  ): SuspiciousActivity[] {
    return this.suspiciousActivities.filter(
      activity => activity.severity === severity
    );
  }

  /**
   * Clears suspicious activity history
   */
  clearSuspiciousActivities(): void {
    this.suspiciousActivities = [];
  }

  /**
   * Updates security configuration
   */
  updateConfig(newConfig: Partial<SecurityConfig>): void {
    this.config = { ...this.config, ...newConfig };
  }

  /**
   * Gets current configuration
   */
  getConfig(): SecurityConfig {
    return { ...this.config };
  }

  /**
   * Adds new pattern for blocking
   */
  addBlockedPattern(pattern: RegExp): void {
    this.config.suspiciousPatterns.push(pattern);
  }

  /**
   * Removes pattern from blocking list
   */
  removeBlockedPattern(pattern: RegExp): void {
    this.config.suspiciousPatterns = this.config.suspiciousPatterns.filter(
      p => p.source !== pattern.source
    );
  }
}

// Export singleton
export const securityManager = SecurityManager.getInstance();
