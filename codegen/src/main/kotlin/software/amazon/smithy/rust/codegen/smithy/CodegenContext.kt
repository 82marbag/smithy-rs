/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rust.codegen.smithy

import software.amazon.smithy.model.Model
import software.amazon.smithy.model.shapes.ServiceShape
import software.amazon.smithy.model.shapes.ShapeId

sealed class CodegenMode {
    object Server : CodegenMode()
    object Client : CodegenMode()
}

/**
 * Configuration needed to generate the client for a given Service<->Protocol pair
 */
data class CodegenContext(
    /**
     * The smithy model.
     *
     * Note: This model may or not be pruned to the given service closure, so ensure that `serviceShape` is used as
     * an entry point.
     */
    val model: Model,
    val symbolProvider: RustSymbolProvider,
    /**
     * Configuration of the runtime package:
     * - Where are the runtime crates (smithy-*) located on the file system? Or are they versioned?
     * - What are they called?
     */
    val runtimeConfig: RuntimeConfig,
    /**
     * Entrypoint service shape for code generation
     */
    val serviceShape: ServiceShape,
    /**
     * Smithy Protocol to generate, e.g. RestJson1
     */
    val protocol: ShapeId,
    /**
     * The name of the cargo crate to generate e.g. `aws-sdk-s3`
     * This is loaded from the smithy-build.json during codegen.
     */
    val moduleName: String,
    /**
     * Settings loaded from smithy-build.json
     */
    val settings: RustSettings,
    /**
     * Server vs. Client codegen
     *
     * Some settings are dependent on whether server vs. client codegen is being invoked.
     */
    val mode: CodegenMode,
) {
    constructor(
        model: Model,
        symbolProvider: RustSymbolProvider,
        serviceShape: ServiceShape,
        protocol: ShapeId,
        settings: RustSettings,
        mode: CodegenMode,
    ) : this(model, symbolProvider, settings.runtimeConfig, serviceShape, protocol, settings.moduleName, settings, mode)
}