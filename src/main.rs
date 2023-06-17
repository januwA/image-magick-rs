use std::{env, ops::Div};

pub type MagickWandPtr = usize;
pub type DrawingWandPtr = usize;
pub type ImagePtr = usize;
pub type PixelWandPtr = usize;
pub type MagickInfoPtr = usize;
pub type KernelInfoPtr = usize;
pub type CCObjectInfoPtr = usize;
pub type VoidPtr = usize;

/* #region enum */
pub type NoiseType = i32;
pub type ExceptionType = i32;
pub type FilterType = i32;
pub type AutoThresholdMethod = i32;
pub type CompositeOperator = i32;
pub type PixelInterpolateMethod = i32;
pub type ColorspaceType = i32;
pub type LayerMethod = i32;
pub type MetricType = i32;
pub type ComplexOperator = i32;
pub type GravityType = i32;
pub type StorageType = i32;
pub type DistortMethod = i32;
pub type MagickEvaluateOperator = i32;
/* #endregion */

// find /usr/local/lib/ -iname "*MagickWand*"
#[link(name = "MagickWand-7.Q16HDRI")]
extern "C" {
    /* #region api magick-wand */

    /**清除与魔杖相关的资源，将魔杖留空，并准备用于一组新的图像 */
    pub fn ClearMagickWand(wand: MagickWandPtr);
    /**制作指定魔杖的精确副本 */
    pub fn CloneMagickWand(wand: MagickWandPtr) -> MagickWandPtr;
    /**释放与 MagickWand 相关的内存 */
    pub fn DestroyMagickWand(wand: MagickWandPtr) -> MagickWandPtr;
    /**如果魔杖被验证为魔杖，则返回 MagickTrue。 */
    pub fn IsMagickWand(wand: MagickWandPtr) -> bool;
    /**清除与魔杖相关的任何异常 */
    pub fn MagickClearException(wand: MagickWandPtr) -> bool;
    /**返回在此 API 中使用其他方法时发生的任何错误的严重性、原因和描述 */
    pub fn MagickGetException(wand: MagickWandPtr, severity: ExceptionType) -> *mut u8;
    /**返回与魔杖关联的异常类型。如果未发生异常，则返回未定义异常类型 */
    pub fn MagickGetExceptionType(wand: MagickWandPtr) -> ExceptionType;
    /**返回迭代器在图像列表中的位置 */
    pub fn MagickGetIteratorIndex(wand: MagickWandPtr) -> isize;
    /**返回与指定配置选项关联的值 */
    pub fn MagickQueryConfigureOption(option: *const u8) -> *mut u8;
    /**返回与指定模式匹配的任何配置选项（例如，“*”表示所有）。选项包括名称、版本、LIB_VERSION等。 */
    pub fn MagickQueryConfigureOptions(
        pattern: *const u8,
        number_options: *mut usize,
    ) -> *mut *mut u8;
    /**返回一个 13 元素数组，表示以下字体指标 */
    pub fn MagickQueryFontMetrics(
        wand: MagickWandPtr,
        drawing_wand: DrawingWandPtr,
        text: *const u8,
    ) -> *mut f64;
    /**返回一个 13 元素数组，表示以下字体指标 */
    pub fn MagickQueryMultilineFontMetrics(
        wand: MagickWandPtr,
        drawing_wand: DrawingWandPtr,
        text: *const u8,
    ) -> *mut f64;
    /**返回与指定模式匹配的任何字体（例如，“*”表示所有字体） */
    pub fn MagickQueryFonts(pattern: *const u8, number_fonts: &mut usize) -> *mut *mut u8;
    /**返回与指定模式匹配的任何图像格式（例如，“*”表示所有） */
    pub fn MagickQueryFormats(pattern: *const u8, number_formats: &mut usize) -> *mut *mut u8;
    /** 放弃由 MagickIdentifyImage（）、MagickGetException（） 等方法返回的内存资源 */
    pub fn MagickRelinquishMemory(resource: &mut usize) -> usize;
    /** 重置魔杖迭代器,它通常在迭代图像之前使用，或者在调用特定函数（如 MagickAppendImages（） ）将所有图像追加在一起之前使用 */
    pub fn MagickResetIterator(wand: MagickWandPtr);
    /**将魔杖迭代器设置为第一个图像 */
    pub fn MagickSetFirstIterator(wand: MagickWandPtr);
    /**将迭代器设置为使用 index 参数指定的图像列表中的给定位置。零索引会将第一个图像设置为当前图像，依此类推。负索引可用于指定相对于魔杖中图像末端的图像，-1 是魔杖中的最后一个图像 */
    pub fn MagickSetIteratorIndex(wand: MagickWandPtr, index: isize) -> bool;
    /**将魔杖迭代器设置为最后一个图像 */
    pub fn MagickSetLastIterator(wand: MagickWandPtr);
    /**初始化 MagickWand 环境 */
    pub fn MagickWandGenesis();
    /**终止 MagickWand 环境 */
    pub fn MagickWandTerminus();
    /**返回 API 中所有其他方法所需的魔杖。如果没有足够的内存来分配魔杖，则会引发致命异常。使用DestroyMagickWand（）在不再需要魔杖时处理它 */
    pub fn NewMagickWand() -> MagickWandPtr;
    /**返回一根带有图像的魔杖 */
    pub fn NewMagickWandFromImage(image: ImagePtr) -> MagickWandPtr;
    /**如果当前实例化了 ImageMagick 环境，则返回 MagickTrue，也就是说，MagickWandGenesis（） 已被调用，但 MagickWandTerminus（） 尚未调用 */
    pub fn IsMagickWandInstantiated() -> bool;

    /* #endregion */

    /* #region api magick */

    /**分配一个 MagickInfo 结构并将成员初始化为默认值 */
    pub fn AcquireMagickInfo(
        magick_module: *const u8,
        name: *const u8,
        description: *const u8,
    ) -> MagickInfoPtr;

    /**返回要打印的最大有效位数 */
    pub fn GetMagickPrecision() -> i32;
    /**如果 ImageMagick 环境尚未实例化，则返回 MagickFalse;当调用MagickCoreGenesis（）但尚未调用MagickDestroy（）时，ImageMagick环境已被实例化 */
    pub fn IsMagickCoreInstantiated() -> bool;
    /**初始化 MagickCore 环境。 */
    pub fn MagickCoreGenesis(path: *const u8, establish_signal_handlers: bool);
    /**破坏了 MagickCore 环境 */
    pub fn MagickCoreTerminus();
    /**设置要打印的最大有效位数 */
    pub fn SetMagickPrecision(precision: i32) -> i32;

    /* #endregion */

    /* #regon api magick-image */

    /**将图像设置为指定的 alpha 级别, 1.0 是完全不透明的，0.0 是完全透明的。 */
    pub fn MagickSetImageAlpha(wand: MagickWandPtr, alpha: f64) -> bool;

    /**从魔杖返回当前图像 */
    pub fn GetImageFromMagickWand(wand: MagickWandPtr) -> ImagePtr;
    /**通过对图像边缘附近不那么强烈的模糊和远离边缘的较强烈的模糊来自适应地模糊图像。我们使用给定半径和标准偏差（sigma）的高斯算子模糊图像。为了获得合理的结果，半径应大于西格玛。使用半径 0，MagickAdaptiveBlurImage（） 会为您选择合适的半径 */
    pub fn MagickAdaptiveBlurImage(wand: MagickWandPtr, radius: f64, sigma: f64) -> bool;
    /**使用数据相关三角测量自适应调整图像大小 */
    pub fn MagickAdaptiveResizeImage(wand: MagickWandPtr, columns: usize, rows: usize) -> bool;
    /**通过在图像边缘附近更强烈地锐化图像，而在远离边缘的较远处锐化强度较低，自适应锐化图像。我们使用给定半径和标准偏差（sigma）的高斯算子锐化图像。为了获得合理的结果，半径应大于西格玛。使用半径 0，MagickAdaptiveSharpenImage（） 会为您选择合适的半径。 */
    pub fn MagickAdaptiveSharpenImage(wand: MagickWandPtr, radius: f64, sigma: f64) -> bool;
    /**根据每个像素的局部邻域中的强度值范围为每个像素选择一个单独的阈值。这允许对其全局强度直方图不包含独特峰的图像进行阈值化 */
    pub fn MagickAdaptiveThresholdImage(
        wand: MagickWandPtr,
        width: usize,
        height: usize,
        bias: f64,
    ) -> bool;
    /**从第二根魔杖中添加图像的克隆，并将它们插入到第一根魔杖中 */
    pub fn MagickAddImage(wand: MagickWandPtr, add_wand: MagickWandPtr) -> bool;
    /**向图像添加随机噪声 */
    pub fn MagickAddNoiseImage(wand: MagickWandPtr, noise_type: NoiseType, attenuate: f64) -> bool;
    /**根据绘图棒的仿射矩阵转换图像 */
    pub fn MagickAffineTransformImage(wand: MagickWandPtr, drawing_wand: DrawingWandPtr) -> bool;
    /**用文本注释图像 */
    pub fn MagickAnnotateImage(
        wand: MagickWandPtr,
        drawing_wand: DrawingWandPtr,
        x: f64,
        y: f64,
        angle: f64,
        text: *const u8,
    ) -> bool;
    /**对图像或图像序列进行动画处理 */
    pub fn MagickAnimateImages(wand: MagickWandPtr, server_name: *const u8) -> bool;
    /**从当前图像开始将图像附加到魔杖中，从而创建一个具有单个图像结果的新魔杖。这受第一个图像的重力和背景设置的影响。 */
    pub fn MagickAppendImages(wand: MagickWandPtr, stack: bool) -> MagickWandPtr;
    /**从图像中提取“平均值”并调整图像以尝试适当地设置其伽玛。 */
    pub fn MagickAutoGammaImage(wand: MagickWandPtr) -> bool;
    /**通过将最小值和最大值缩放到整个量子范围来调整特定图像通道的级别。 */
    pub fn MagickAutoLevelImage(wand: MagickWandPtr) -> bool;
    /**调整图像，使其方向适合查看 $（即左上角方向） */
    pub fn MagickAutoOrientImage(image: MagickWandPtr) -> bool;
    /**根据您指定的方法自动执行图像阈值 */
    pub fn MagickAutoThresholdImage(wand: MagickWandPtr, method: AutoThresholdMethod) -> bool;
    /**是一种非线性、边缘保留和降噪的图像平滑过滤器。它将每个像素的强度替换为附近像素的强度值的加权平均值。此权重基于高斯分布。权重不仅取决于像素的欧几里得距离，还取决于辐射差异（例如，范围差异，例如颜色强度、深度距离等）。这样可以保留锋利的边缘。 */
    pub fn MagickBilateralBlurImage(
        wand: MagickWandPtr,
        radius: f64,
        sigma: f64,
        intensity_sigma: f64,
        spatial_sigma: f64,
    ) -> bool;
    /**类似于 MagickThresholdImage（），但强制低于阈值的所有像素变为黑色，同时保持高于阈值的所有像素不变。 */
    pub fn MagickBlackThresholdImage(wand: MagickWandPtr, threshold: PixelWandPtr) -> bool;
    /**将图像的颜色静音，以模拟夜间月光下的场景 */
    pub fn MagickBlueShiftImage(wand: MagickWandPtr, factor: f64) -> bool;
    /**模糊图像。我们使用给定半径和标准偏差（sigma）的高斯算子对图像进行卷积。为了获得合理的结果，半径应大于西格玛。使用半径 0，BlurImage（） 会为您选择合适的半径。 */
    pub fn MagickBlurImage(wand: MagickWandPtr, radius: f64, sigma: f64) -> bool;
    /**用边框像素棒定义的颜色边框包围图像 */
    pub fn MagickBorderImage(
        wand: MagickWandPtr,
        bordercolor: PixelWandPtr,
        width: usize,
        height: usize,
        compose: CompositeOperator,
    ) -> bool;
    /**更改图像的亮度和/或对比度。它将亮度和对比度参数转换为斜率和截距，并调用多项式函数以应用于图像。 */
    pub fn MagickBrightnessContrastImage(
        wand: MagickWandPtr,
        brightness: f64,
        contrast: f64,
    ) -> bool;
    /**使用多阶段算法来检测图像中的各种边缘。 */
    pub fn MagickCannyEdgeImage(
        wand: MagickWandPtr,
        radius: f64,
        sigma: f64,
        lower_percent: f64,
        upper_percent: f64,
    ) -> bool;
    /**将通道表达式应用于指定的图像。表达式由一个或多个通道组成，助记符或数字（例如红色，1），由操作分隔 */
    pub fn MagickChannelFxImage(wand: MagickWandPtr, expression: *const u8) -> MagickWandPtr;
    /**模拟木炭图 */
    pub fn MagickCharcoalImage(wand: MagickWandPtr, radius: i32, sigma: i32) -> bool;
    /**删除图像的一个区域并折叠图像以占据删除的部分 */
    pub fn MagickChopImage(
        wand: MagickWandPtr,
        width: usize,
        height: usize,
        x: isize,
        y: isize,
    ) -> bool;
    /**是自适应直方图均衡的一种变体，其中对比度放大受到限制，以减少噪声放大的问题 */
    pub fn MagickCLAHEImage(
        wand: MagickWandPtr,
        width: usize,
        height: usize,
        number_bins: f64,
        clip_limit: f64,
    ) -> bool;
    /**将颜色范围限制在 0 到量子深度之间 */
    pub fn MagickClampImage(wand: MagickWandPtr) -> bool;
    /**沿着 8BIM 配置文件中的第一条路径（如果存在）进行剪辑。 */
    pub fn MagickClipImage(wand: MagickWandPtr) -> bool;
    /**沿着 8BIM 配置文件中的命名路径（如果存在）进行剪辑。后续操作在路径内生效。如果前面有#，Id可以是一个数字，用于处理编号路径，例如，“#1”使用第一个路径。 */
    pub fn MagickClipImagePath(wand: MagickWandPtr, pathname: *const u8, inside: bool) -> bool;
    /** 从颜色查找表中替换图像中的颜色。 */
    pub fn MagickClutImage(
        wand: MagickWandPtr,
        clut_wand: MagickWandPtr,
        method: PixelInterpolateMethod,
    ) -> bool;
    /**复合一组图像，同时尊重任何页面偏移和处理方法。GIF、MIFF 和 MNG 动画序列通常以图像背景开始，每个后续图像的大小和偏移量各不相同。MagickCoalesceImages（） 返回一个新序列，其中序列中的每个图像与第一个图像的大小相同，并与序列中的下一个图像合成。 */
    pub fn MagickCoalesceImages(wand: MagickWandPtr) -> MagickWandPtr;
    /** 接受一个轻量级色彩校正集合 （CCC） 文件，该文件仅包含一个或多个颜色校正，并将颜色校正应用于图像。下面是一个示例 CCC 文件 */
    pub fn MagickColorDecisionListImage(
        wand: MagickWandPtr,
        color_correction_collection: *const u8,
    ) -> bool;
    /**将填充颜色与图像中的每个像素混合在一起 */
    pub fn MagickColorizeImage(
        wand: MagickWandPtr,
        colorize: PixelWandPtr,
        blend: PixelWandPtr,
    ) -> bool;
    /**对图像应用颜色转换。该方法允许饱和度变化、色调旋转、亮度到 alpha 以及各种其他效果。虽然可以使用可变大小的变换矩阵，但通常对 RGBA 图像使用 5x5 矩阵，对 CMYKA（或带偏移的 RGBA）使用 6x6 矩阵。该矩阵与Adobe Flash使用的矩阵类似，只是偏移量位于第6列而不是第5列（支持CMYKA图像），偏移量被规范化（将Flash偏移量除以255）。 */
    pub fn MagickColorMatrixImage(wand: MagickWandPtr, color_matrix: KernelInfoPtr) -> bool;
    /**强制颜色范围内的所有像素为白色，否则为黑色。 */
    pub fn MagickColorThresholdImage(
        wand: MagickWandPtr,
        start_color: PixelWandPtr,
        stop_color: PixelWandPtr,
    ) -> bool;
    /**将一个或多个图像组合成一个图像。序列中每个图像的像素的灰度值被分配给组合图像的指定通道。典型的顺序是图像 1 => 红色、2 => 绿色、3 => 蓝色等 */
    pub fn MagickCombineImages(wand: MagickWandPtr, colorspace: ColorspaceType) -> MagickWandPtr;
    /**为您的图像添加注释 */
    pub fn MagickCommentImage(wand: MagickWandPtr, comment: *const u8) -> bool;

    /** 将每个图像与序列中的下一个图像进行比较，并返回它发现的任何像素差异的最大边界区域。 */
    pub fn MagickCompareImagesLayers(wand: MagickWandPtr, method: LayerMethod) -> MagickWandPtr;

    /**将图像与重建的图像进行比较，并返回指定的差异图像。 */
    pub fn MagickCompareImages(
        wand: MagickWandPtr,
        reference: MagickWandPtr,
        metric: MetricType,
        distortion: *mut f64,
    ) -> MagickWandPtr;

    /**对图像序列执行复杂的数学运算。 */
    pub fn MagickComplexImages(wand: MagickWandPtr, op: ComplexOperator) -> MagickWandPtr;

    /** 以指定的偏移量将一个图像合成到另一个图像上 */
    pub fn MagickCompositeImage(
        wand: MagickWandPtr,
        source_wand: MagickWandPtr,
        compose: CompositeOperator,
        clip_to_self: bool,
        x: isize,
        y: isize,
    ) -> bool;

    /**使用指定的重力将一个图像合成到另一个图像上。 */
    pub fn MagickCompositeImageGravity(
        wand: MagickWandPtr,
        source_wand: MagickWandPtr,
        compose: CompositeOperator,
        gravity: GravityType,
    ) -> bool;
    /**将源魔杖中的图像按顺序合成到目标魔杖中的图像上，从两个列表中的当前图像开始。 */
    pub fn MagickCompositeLayers(
        wand: MagickWandPtr,
        source_wand: MagickWandPtr,
        compose: CompositeOperator,
        x: isize,
        y: isize,
    ) -> bool;
    /**返回唯一标记的图像的连接组件。返回的连接组件图像颜色成员定义唯一对象的数量。从 4 路或 8 路连接中进行选择。 */
    pub fn MagickConnectedComponentsImage(
        wand: MagickWandPtr,
        connectivity: usize,
        objects: *mut CCObjectInfoPtr,
    ) -> bool;
    /**增强了图像中较亮和较暗元素之间的强度差异。将锐化设置为 0 以外的值以增加图像对比度，否则对比度会降低。 */
    pub fn MagickContrastImage(wand: MagickWandPtr, sharpen: bool) -> bool;

    /**通过调整像素颜色以跨越整个可用颜色范围来增强彩色图像的对比度。您还可以减少 Gamma 值为 0 的特定通道的影响。 */
    pub fn MagickContrastStretchImage(
        wand: MagickWandPtr,
        black_point: f64,
        white_point: f64,
    ) -> bool;

    /**将自定义卷积内核应用于图像 */
    pub fn MagickConvolveImage(wand: MagickWandPtr, kernel: KernelInfoPtr) -> bool;

    /**提取图像的一个区域 */
    pub fn MagickCropImage(
        wand: MagickWandPtr,
        width: usize,
        height: usize,
        x: isize,
        y: isize,
    ) -> bool;
    /**将图像的颜色图替换给定数量的位置。如果你多次循环颜色图，你会产生心理效果 */
    pub fn MagickCycleColormapImage(wand: MagickWandPtr, displace: isize) -> bool;

    /**将图像添加到由您提供的像素数据组成的魔杖中。像素数据必须按扫描线顺序从上到下排列。数据可以是字符、短整型、整型、浮点数或双精度型。浮点数和双精度要求像素规范化 [0..1]，否则 [0..Max]，其中 Max 是类型可以容纳的最大值（例如，字符为 255）。例如，要从未签名的红绿蓝字符数据创建 640x480 图像， */
    pub fn MagickConstituteImage(
        wand: MagickWandPtr,
        columns: usize,
        rows: usize,
        map: *const u8,
        storage: StorageType,
        pixels: VoidPtr,
    ) -> bool;
    /**将密码像素转换为普通像素。 */
    pub fn MagickDecipherImage(wand: MagickWandPtr, passphrase: *const u8) -> bool;
    /** 将每个图像与序列中的下一个图像进行比较，并返回它发现的任何像素差异的最大边界区域。 */
    pub fn MagickDeconstructImages(wand: MagickWandPtr) -> MagickWandPtr;
    /**从图像中删除了倾斜。歪斜是一种伪影，由于相机未对准、扫描或表面出现缺陷，或者仅仅是因为纸张在扫描时未完全平放而出现在扫描图像中。 */
    pub fn MagickDeskewImage(wand: MagickWandPtr, threshold: f64) -> bool;
    /**可减少图像中的斑点噪点，同时保留原始图像的边缘 */
    pub fn MagickDespeckleImage(wand: MagickWandPtr) -> bool;
    /** 取消引用图像，如果引用计数变为零，则解除分配与图像关联的内存。 */
    pub fn MagickDestroyImage(image: ImagePtr) -> ImagePtr;
    /**显示一个图像。 */
    pub fn MagickDisplayImage(wand: MagickWandPtr, server_name: *const u8) -> bool;
    /**显示图像或图像序列。 */
    pub fn MagickDisplayImages(wand: MagickWandPtr, server_name: *const u8) -> bool;
    /**使用各种失真方法扭曲图像，方法是将源图像的颜色查找映射到通常与源图像大小相同的新目标图像，除非将“最佳拟合”设置为 true。

    如果启用了“最佳拟合”，并且失真允许，则会调整目标图像以确保整个源“图像”正好适合最终目标图像，最终目标图像将相应地调整大小和偏移。此外，在许多情况下，映射中将考虑源图像的虚拟偏移量。 */
    pub fn MagickDistortImage(
        wand: MagickWandPtr,
        method: DistortMethod,
        number_arguments: usize,
        arguments: *mut f64,
        bestfit: bool,
    ) -> bool;
    /**在当前图像上渲染绘图棒 */
    pub fn MagickDrawImage(wand: MagickWandPtr, drawing_wand: DrawingWandPtr) -> bool;
    /**使用给定半径的卷积滤波器增强图像中的边缘。使用 0 的半径，Edge（） 会为您选择合适的半径。 */
    pub fn MagickEdgeImage(wand: MagickWandPtr, radius: f64) -> bool;
    /**返回具有三维效果的灰度图像。我们使用给定半径和标准偏差（sigma）的高斯算子对图像进行卷积。为了获得合理的结果，半径应大于西格玛。使用半径 0，Emboss（） 会为您选择合适的半径。 */
    pub fn MagickEmbossImage(wand: MagickWandPtr, radius: f64, sigma: f64) -> bool;
    /**将普通像素转换为密码像素。 */
    pub fn MagickEncipherImage(wand: MagickWandPtr, passphrase: *const u8) -> bool;
    /**应用数字滤波器来提高噪点图像的质量。 */
    pub fn MagickEnhanceImage(wand: MagickWandPtr) -> bool;
    /**均衡图像直方图 */
    pub fn MagickEqualizeImage(wand: MagickWandPtr) -> bool;
    /**将算术、关系或逻辑表达式应用于图像。使用这些运算符可以使图像变亮或变暗，增加或减少图像中的对比度，或生成图像的“负片”。 */
    pub fn MagickEvaluateImage(
        wand: MagickWandPtr,
        operator: MagickEvaluateOperator,
        value: f64,
    ) -> bool;
    pub fn MagickEvaluateImages(wand: MagickWandPtr, operator: MagickEvaluateOperator) -> bool;

    /**从图像中提取像素数据并将其返回给您。该方法在成功时返回 MagickTrue，否则如果遇到错误，则返回 MagickFalse。数据按 map 指定的顺序以字符、短整型、整型、ssize_t、浮点数或双精度数的形式返回。 */
    pub fn MagickExportImagePixels(
        wand: MagickWandPtr,
        x: isize,
        y: isize,
        columns: usize,
        rows: usize,
        map: *const u8,
        storage: StorageType,
        pixels: VoidPtr,
    ) -> bool;
    /**扩展了由几何图形、重力和魔杖背景颜色定义的图像。设置几何图形的 （x，y） 偏移量以相对于扩展的魔杖移动原始魔杖。 */
    pub fn MagickExtentImage(
        wand: MagickWandPtr,
        width: usize,
        height: usize,
        x: isize,
        y: isize,
    ) -> bool;
    /**通过围绕中心 x 轴反射像素来创建垂直镜像。 */
    pub fn MagickFlipImage(wand: MagickWandPtr) -> bool;
    /**更改与目标匹配且是直接邻居的任何像素的颜色值。如果指定了方法 FillToBorderMethod，则会更改与图像的边框颜色成员不匹配的任何相邻像素的颜色值。 */
    pub fn MagickFloodfillPaintImage(
        wand: MagickWandPtr,
        fill: PixelWandPtr,
        fuzz: f64,
        bordercolor: PixelWandPtr,
        x: isize,
        y: isize,
        invert: bool,
    ) -> bool;
    /**通过围绕中心 y 轴反射像素来创建水平镜像。 */
    pub fn MagickFlopImage(wand: MagickWandPtr) -> bool;
    /**将图像的离散傅里叶变换 （DFT） 实现为幅度/相位或实数/虚构图像对。 */
    pub fn MagickForwardFourierTransformImage(wand: MagickWandPtr, magnitude: bool) -> bool;
    /**在图像周围添加了一个模拟的三维边框。宽度和高度指定框架的垂直边和水平边的边框宽度。内斜面和外斜面表示框架的内阴影和外阴影的宽度。 */
    pub fn MagickFrameImage(
        wand: MagickWandPtr,
        matte_color: PixelWandPtr,
        width: usize,
        height: usize,
        inner_bevel: isize,
        outer_bevel: isize,
        compose: CompositeOperator,
    ) -> bool;
    /**返回图像宽度。 */
    pub fn MagickGetImageWidth(wand: MagickWandPtr) -> usize;
    /**返回图像高度。 */
    pub fn MagickGetImageHeight(wand: MagickWandPtr) -> usize;
    /**将指定大小和背景颜色的空白图像画布添加到魔杖中。 */
    pub fn MagickNewImage(
        wand: MagickWandPtr,
        columns: usize,
        rows: usize,
        background: PixelWandPtr,
    ) -> bool;
    /**写入图像或图像序列。 */
    pub fn MagickWriteImages(wand: MagickWandPtr, filename: *const u8, adjoin: bool) -> bool;
    /**将图像写入指定的文件名。如果文件名参数为 NULL，则图像将写入由 MagickReadImage（） 或 MagickSetImageFilename（） 设置的文件名。 */
    pub fn MagickWriteImage(wand: MagickWandPtr, filename: *const u8) -> bool;
    /**读取图像或图像序列。图像将插入到当前图像指针位置的正前方 */
    pub fn MagickReadImage(wand: MagickWandPtr, filename: *const u8) -> bool;
    /* #endregion */

    /* #regon api pixel-wand */

    /**清除与魔杖关联的资源。 */
    pub fn ClearPixelWand(wand: PixelWandPtr);
    /**制作指定魔杖的精确副本 */
    pub fn ClonePixelWand(wand: PixelWandPtr) -> PixelWandPtr;
    /**制作指定魔杖的精确副本。 */
    pub fn ClonePixelWands(wand: *mut PixelWandPtr, number_wands: usize) -> *mut PixelWandPtr;
    /**释放与 PixelWand 相关的资源 */
    pub fn DestroyPixelWand(wand: PixelWandPtr) -> PixelWandPtr;
    pub fn DestroyPixelWands(wand: *mut PixelWandPtr, number_wands: usize) -> *mut PixelWandPtr;
    /**如果两种颜色之间的距离小于指定的距离，则返回 MagickTrue。 */
    pub fn IsPixelWandSimilar(p: PixelWandPtr, q: PixelWandPtr, fuzz: f64) -> bool;
    /**返回一个新的像素棒。 */
    pub fn NewPixelWand() -> PixelWandPtr;
    /**用字符串设置像素棒的颜色（例如“蓝色”、“#0000ff”、“rgb（0，0，255）”、“cmyk（100，100，100，10）”等）。 */
    pub fn PixelSetColor(wand: PixelWandPtr, color: *const u8) -> bool;

    /* #endregion */

    /* #regon api drawing-wand */

    pub fn NewDrawingWand() -> DrawingWandPtr;
    pub fn DestroyDrawingWand(wand: DrawingWandPtr) -> DrawingWandPtr;
    /**在图像上画一个圆。 */
    pub fn DrawCircle(wand: DrawingWandPtr, ox: f64, oy: f64, px: f64, py: f64);
    /**设置用于绘制填充对象的填充颜色。 */
    pub fn DrawSetFillColor(wand: DrawingWandPtr, fill_wand: PixelWandPtr);
    /* #endregion */

    pub fn MagickNextImage(wand: MagickWandPtr) -> bool;
    pub fn MagickResizeImage(wand: MagickWandPtr, w: usize, h: usize, filter: FilterType) -> bool;

}

pub struct MagickWandContext {}
impl MagickWandContext {
    pub fn new() {
        unsafe {
            MagickWandGenesis();
        }
    }
}
impl Drop for MagickWandContext {
    fn drop(&mut self) {
        println!("MagickWandTerminus");
        unsafe {
            MagickWandTerminus();
        }
    }
}

pub struct MagickWand {
    pub wand: MagickWandPtr,
}
impl MagickWand {
    pub fn new() -> Self {
        unsafe {
            let wand = NewMagickWand();
            MagickWand { wand }
        }
    }
}
impl Drop for MagickWand {
    fn drop(&mut self) {
        if self.wand != 0 {
            println!("DestroyMagickWand");
            unsafe {
                DestroyMagickWand(self.wand);
            }
        }
    }
}

pub struct PixelWand {
    pub wand: PixelWandPtr,
}
impl PixelWand {
    pub fn new() -> Self {
        unsafe {
            let wand = NewPixelWand();
            PixelWand { wand }
        }
    }
}
impl Drop for PixelWand {
    fn drop(&mut self) {
        if self.wand != 0 {
            println!("DestroyPixelWand");
            unsafe {
                DestroyPixelWand(self.wand);
            }
        }
    }
}

pub struct DrawingWand {
    pub wand: DrawingWandPtr,
}
impl DrawingWand {
    pub fn new() -> Self {
        unsafe {
            let wand = NewDrawingWand();
            DrawingWand { wand }
        }
    }
}
impl Drop for DrawingWand {
    fn drop(&mut self) {
        if self.wand != 0 {
            println!("DestroyDrawingWand");
            unsafe {
                DestroyDrawingWand(self.wand);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", &args);

    circle(&args);
}

// cargo r -- ./a.jpg ./o.jpg 100
fn resize(args: &Vec<String>) {
    unsafe {
        MagickWandContext::new();

        let mw = MagickWand::new();

        let inpath = &args[1];
        if !MagickReadImage(mw.wand, inpath.as_ptr()) {
            return;
        }

        MagickResetIterator(mw.wand);

        let wh = args[3].parse::<usize>().unwrap_or(100);

        while MagickNextImage(mw.wand) != false {
            if !MagickResizeImage(mw.wand, wh, wh, 22) {
                return;
            }
        }

        let outpath = &args[2];
        MagickWriteImages(mw.wand, outpath.as_ptr(), true);
    }
}

fn circle1(args: &Vec<String>) {
    unsafe {
        MagickWandContext::new();

        let bg_wand = MagickWand::new();
        let draw_wand = DrawingWand::new();
        let color_wand = PixelWand::new();
        dbg!(bg_wand.wand);
        dbg!(draw_wand.wand);
        dbg!(color_wand.wand);

        let diameter = 640;
        let radius = diameter / 2;

        // 在c中直接写名称有效，但是这里不行
        // dbg!(PixelSetColor(color_wand.wand, "transparent".as_ptr()));
        dbg!(PixelSetColor(color_wand.wand, "rgba(0,0,0,0)".as_ptr()));
        dbg!(MagickNewImage(
            bg_wand.wand,
            diameter,
            diameter,
            color_wand.wand
        ));
        // dbg!(MagickSetImageAlpha(bg_wand.wand, 0.0));

        dbg!(PixelSetColor(color_wand.wand, "#ff0000".as_ptr()));
        DrawSetFillColor(draw_wand.wand, color_wand.wand);
        DrawCircle(
            draw_wand.wand,
            radius as f64,
            radius as f64,
            radius as f64,
            (radius * 2) as f64,
        );

        dbg!(MagickDrawImage(bg_wand.wand, draw_wand.wand));
        dbg!(MagickWriteImage(bg_wand.wand, args[1].as_ptr()));
    }
}

// cargo r -- ./a.jpg ./o.png 17
fn circle(args: &Vec<String>) {
    unsafe {
        MagickWandContext::new();

        let bg_wand = MagickWand::new();
        let circle_wand = MagickWand::new();
        let color_wand = PixelWand::new();
        let draw_wand = DrawingWand::new();

        if !MagickReadImage(bg_wand.wand, args[1].as_ptr()) {
            return;
        }

        let w = MagickGetImageWidth(bg_wand.wand);
        let h = MagickGetImageHeight(bg_wand.wand);
        let r2 = if w < h { w } else { h };
        let r = f64::from(r2 as u32).div(2.0);
        println!("{w} {h} {r2} {r}");

        // create image draw black circle
        PixelSetColor(color_wand.wand, "rgba(0,0,0,0)".as_ptr());
        MagickNewImage(circle_wand.wand, w, h, color_wand.wand);

        PixelSetColor(color_wand.wand, "#000".as_ptr());
        DrawCircle(draw_wand.wand, r, r, r, r * 2.0);
        DrawSetFillColor(draw_wand.wand, color_wand.wand);
        MagickDrawImage(circle_wand.wand, draw_wand.wand);

        // CopyAlphaCompositeOp = 17 裁剪一个圆形
        // CompositeOperator op = (CompositeOperator)strtoul(argv[3], NULL, 10);

        let op: i32 = args[3].parse::<i32>().unwrap();

        MagickResetIterator(bg_wand.wand);
        while MagickNextImage(bg_wand.wand) != false {
            MagickCompositeImage(bg_wand.wand, circle_wand.wand, op, true, 0, 0);
            MagickResizeImage(bg_wand.wand, 140, 140, 0);
        }

        MagickWriteImages(bg_wand.wand, args[2].as_ptr(), true);
    }
}
