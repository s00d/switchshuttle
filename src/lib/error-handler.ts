import { SwitchShuttleCommands } from './tauri-commands';

// Types for error handling
export interface AppError {
  message: string;
  code?: string;
  details?: any;
  timestamp: Date;
  context?: string;
  severity?: 'low' | 'medium' | 'high' | 'critical';
}

export interface ErrorContext {
  component?: string;
  action?: string;
  userId?: string;
  sessionId?: string;
  severity?: 'low' | 'medium' | 'high' | 'critical';
}

export interface Notification {
  title: string;
  body: string;
  type?: 'default' | 'success' | 'error' | 'info' | 'warning';
  icon?: string;
}

export type NotificationType =
  | 'default'
  | 'success'
  | 'error'
  | 'info'
  | 'warning';

export class ErrorHandler {
  private static instance: ErrorHandler;
  private errors: AppError[] = [];
  private maxErrors = 100; // Maximum number of errors to store

  private constructor() {}

  static getInstance(): ErrorHandler {
    if (!ErrorHandler.instance) {
      ErrorHandler.instance = new ErrorHandler();
    }
    return ErrorHandler.instance;
  }

  /**
   * Handles an error and shows a notification to the user
   */
  async handleError(
    error: Error | string | AppError,
    context?: string | ErrorContext
  ): Promise<void> {
    const appError: AppError = {
      message: typeof error === 'string' ? error : error.message,
      code: (error as any)?.code,
      details: error,
      timestamp: new Date(),
      context: typeof context === 'string' ? context : context?.component,
      severity: this.determineSeverity(error),
    };

    this.errors.push(appError);

    // Limit the number of errors
    if (this.errors.length > this.maxErrors) {
      this.errors = this.errors.slice(-this.maxErrors);
    }

    // Log the error
    console.error(`[${appError.context || 'App'}] Error:`, appError);

    // Show notification to user
    try {
      const notificationType = this.getNotificationType(appError.severity);
      await this.showNotification(appError, notificationType);
    } catch (notificationError) {
      console.error('Failed to show error notification:', notificationError);
    }
  }

  /**
   * Handles network errors
   */
  async handleNetworkError(error: Error, operation: string): Promise<void> {
    const message = `Network error while performing operation "${operation}": ${error.message}`;
    await this.handleError(message, {
      component: 'Network',
      action: operation,
    });
  }

  /**
   * Handles validation errors
   */
  async handleValidationError(field: string, message: string): Promise<void> {
    const errorMessage = `Validation error for field "${field}": ${message}`;
    await this.handleError(errorMessage, {
      component: 'Validation',
      action: 'validate_field',
    });
  }

  /**
   * Handles Tauri command errors
   */
  async handleTauriError(error: Error, command: string): Promise<void> {
    const message = `Error executing command "${command}": ${error.message}`;
    await this.handleError(message, {
      component: 'Tauri',
      action: command,
    });
  }

  /**
   * Handles security errors
   */
  async handleSecurityError(error: Error, action: string): Promise<void> {
    const message = `Security error while performing "${action}": ${error.message}`;
    await this.handleError(message, {
      component: 'Security',
      action: action,
      severity: 'high',
    });
  }

  /**
   * Handles performance errors
   */
  async handlePerformanceError(error: Error, metric: string): Promise<void> {
    const message = `Performance error "${metric}": ${error.message}`;
    await this.handleError(message, {
      component: 'Performance',
      action: metric,
      severity: 'medium',
    });
  }

  /**
   * Determines error severity
   */
  private determineSeverity(
    error: Error | string | AppError
  ): 'low' | 'medium' | 'high' | 'critical' {
    if (typeof error === 'string') {
      return 'medium';
    }

    if ('severity' in error && error.severity) {
      return error.severity;
    }

    // Determine by error type
    const errorMessage = error.message.toLowerCase();
    if (
      errorMessage.includes('security') ||
      errorMessage.includes('permission')
    ) {
      return 'high';
    }
    if (
      errorMessage.includes('network') ||
      errorMessage.includes('connection')
    ) {
      return 'medium';
    }
    if (
      errorMessage.includes('validation') ||
      errorMessage.includes('format')
    ) {
      return 'low';
    }

    return 'medium';
  }

  /**
   * Gets notification type based on error severity
   */
  private getNotificationType(severity?: string): NotificationType {
    switch (severity) {
      case 'critical':
      case 'high':
        return 'error';
      case 'medium':
        return 'warning';
      case 'low':
        return 'info';
      default:
        return 'error';
    }
  }

  /**
   * Shows notification to user
   */
  private async showNotification(
    error: AppError,
    type: NotificationType
  ): Promise<void> {
    const notification: Notification = {
      title: this.getNotificationTitle(type),
      body: error.message,
      type: type,
      icon: this.getNotificationIcon(type),
    };

    try {
      switch (type) {
        case 'error':
          await SwitchShuttleCommands.show_error_notification(
            notification.title,
            notification.body
          );
          break;
        case 'warning':
          await SwitchShuttleCommands.show_warning_notification(
            notification.title,
            notification.body
          );
          break;
        case 'success':
          await SwitchShuttleCommands.show_success_notification(
            notification.title,
            notification.body
          );
          break;
        default:
          await SwitchShuttleCommands.show_info_notification(
            notification.title,
            notification.body
          );
      }
    } catch (notificationError) {
      console.error('Failed to show notification:', notificationError);
    }
  }

  /**
   * Gets notification title
   */
  private getNotificationTitle(type: NotificationType): string {
    switch (type) {
      case 'error':
        return 'Application Error';
      case 'warning':
        return 'Warning';
      case 'success':
        return 'Success';
      case 'info':
        return 'Information';
      default:
        return 'Notification';
    }
  }

  /**
   * Gets notification icon
   */
  private getNotificationIcon(type: NotificationType): string {
    switch (type) {
      case 'error':
        return '❌';
      case 'warning':
        return '⚠️';
      case 'success':
        return '✅';
      case 'info':
        return 'ℹ️';
      default:
        return '📢';
    }
  }

  /**
   * Gets all errors
   */
  getErrors(): AppError[] {
    return [...this.errors];
  }

  /**
   * Gets errors by severity
   */
  getErrorsBySeverity(
    severity: 'low' | 'medium' | 'high' | 'critical'
  ): AppError[] {
    return this.errors.filter(error => error.severity === severity);
  }

  /**
   * Gets errors by context
   */
  getErrorsByContext(context: string): AppError[] {
    return this.errors.filter(error => error.context === context);
  }

  /**
   * Clears error history
   */
  clearErrors(): void {
    this.errors = [];
  }

  /**
   * Gets the last error
   */
  getLastError(): AppError | null {
    return this.errors.length > 0 ? this.errors[this.errors.length - 1] : null;
  }

  /**
   * Gets error statistics
   */
  getErrorStats(): {
    total: number;
    bySeverity: Record<string, number>;
    byContext: Record<string, number>;
  } {
    const bySeverity: Record<string, number> = {};
    const byContext: Record<string, number> = {};

    this.errors.forEach(error => {
      // Count by severity
      const severity = error.severity || 'medium';
      bySeverity[severity] = (bySeverity[severity] || 0) + 1;

      // Count by context
      const context = error.context || 'unknown';
      byContext[context] = (byContext[context] || 0) + 1;
    });

    return {
      total: this.errors.length,
      bySeverity,
      byContext,
    };
  }
}

// Export singleton
export const errorHandler = ErrorHandler.getInstance();
